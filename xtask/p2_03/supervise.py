"""Execute one frozen P2-03 stage, preserving first cause and complete argv."""
from decimal import Decimal
import fcntl
import importlib.util
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time
from run_evidence import FinishRecord, build_limit, read, sha, verify_receipts, write


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def check_pins(plan):
    for filename, expected in read(plan["pins"]).items():
        if sha(filename) != expected:
            raise ValueError(f"frozen input changed: {filename}")


def check_rust(text, selected):
    cases = re.findall(r"(?m)^test (\S+) \.\.\. (\S+)\s*$", text)
    if not selected or sorted(name for name, _ in cases) != sorted(selected):
        raise ValueError("Rust selection is empty or differs from the frozen list")
    if any(result != "ok" for _, result in cases):
        raise ValueError("Rust selection failed")


def worker(plan, stage):
    folder = Path(plan["output"]) / stage
    steps = plan["stages"][stage]["steps"]
    actual = []
    for row in steps:
        with (folder / (row["name"] + ".stdout")).open("xb") as out, (folder / (row["name"] + ".stderr")).open("xb") as err:
            child = subprocess.run(row["argv"], cwd=plan["cwd"],
                                   env={**os.environ, **row.get("env", {})}, stdout=out, stderr=err)
        receipt = {"name": row["name"], "argv": row["argv"], "exit": child.returncode}
        actual.append(receipt)
        write(folder / "commands.json", actual)
        if child.returncode:
            return child.returncode
        if "selectedTests" in row:
            try:
                check_rust((folder / (row["name"] + ".stdout")).read_text(), row["selectedTests"])
            except ValueError as error:
                receipt["coverageError"] = str(error)
                write(folder / "commands.json", actual)
                return 1
    return 0


def frozen_roots(root, mutable):
    for path in root.iterdir():
        if any(path == target for target in mutable):
            continue
        if path.is_dir() and any(target.is_relative_to(path) for target in mutable):
            yield from frozen_roots(path, mutable)
        else:
            yield path


def run(plan_path, plan, stage):
    config = plan["stages"][stage]
    auth = read(config["authorization"])
    if sha(config["authorization"]) != config["authorizationSHA256"] or not auth["approved"]:
        raise ValueError("stage authorization is missing or changed")
    if auth["candidateSHA256"] != plan["candidateSHA256"] or auth["stage"] != stage:
        raise ValueError("stage authorization belongs to another candidate")
    before = read(plan["inheritedBudget"])
    before_time = {"card": before["cumulative"]["cardSeconds"],
                   "continuation": before["cumulative"]["continuationSeconds"],
                   "package": before["packageSecondsUsed"]}
    cap = Decimal(str(config["capSeconds"]))
    if config["class"] == "build":
        window = read(auth["windowClarification"])
        if sha(auth["windowClarification"]) != auth["windowClarificationSHA256"]:
            raise ValueError("window clarification changed")
        build_limit({"windowSeconds": 1783, "hardBuildInvocationCap": window["hardBuildInvocationCapInWindow"],
                     "usedSeconds": before["cumulative"]["builds"]["windowSecondsUsed"]},
                    before_time, str(cap))
    else:
        # A new user grant must explicitly raise the exhausted test-group cap.
        if auth["testGroupNumber"] <= before["cumulative"]["testGroups"]["used"] or cap > 600:
            raise ValueError("test group does not add an authorized slot")
        if auth["testGroupLimit"] < auth["testGroupNumber"]:
            raise ValueError("test group exceeds the new cap")
    if config["capSeconds"] != auth["capSeconds"]:
        raise ValueError("stage time limit differs from its grant")
    for key, limit in [("card", 14400), ("continuation", 10800), ("package", 3000)]:
        if Decimal(before_time[key]) + cap > limit:
            raise ValueError("stage reservation exceeds inherited budget")
    check_pins(plan)
    monitor = load(Path(plan["monitor"]), "p203_frozen_monitor")
    with Path(plan["lock"]).open("rb") as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        folder = Path(plan["output"]) / stage
        folder.mkdir(parents=True, exist_ok=False)
        started = time.monotonic()
        ledger = {"state": "reserved", "stage": stage, "class": config["class"],
                  "seconds": str(cap), "capSeconds": str(cap),
                  "matrixSHA256": sha(plan_path), "candidateSHA256": plan["candidateSHA256"],
                  "authorizationSHA256": sha(config["authorization"]),
                  "sourcePinsSHA256": sha(plan["pins"]),
                  "profileSHA256": sha(config["profile"]), "samples": []}
        record = FinishRecord(folder / "ledger.json", ledger)
        write(folder / "ledger.json", ledger)
        child = None
        mutable = list(map(Path, plan["mutableRoots"]))
        historical = None
        def stop(sig, _frame):
            raise InterruptedError(f"received signal {sig}")
        handlers = {sig: signal.signal(sig, stop) for sig in (signal.SIGINT, signal.SIGTERM)}
        try:
            historical = monitor.disk_bytes(list(frozen_roots(Path(plan["p2Root"]), mutable)), timeout=30)
            disk = historical + monitor.disk_bytes(mutable, timeout=30)
            if disk > monitor.DISK_LIMIT:
                raise ValueError("disk budget before launch")
            scratch = folder / "tmp"
            scratch.mkdir()
            env = {key: value for key, value in os.environ.items()
                   if not key.startswith(("CARGO_", "RUST", "VESTA_", "NODE_", "PYTHON", "DYLD_"))
                   and "proxy" not in key.lower()}
            env.update(plan["env"], TMPDIR=str(scratch) + "/")
            env["PATH"] = str(Path(plan["env"]["RUSTC"]).parent) + ":" + env.get("PATH", "")
            argv = ["/usr/bin/sandbox-exec", "-D", f"STAGE_ROOT={folder}"]
            if config["class"] == "build":
                argv += ["-D", f"CARGO_CACHE={plan['env']['CARGO_HOME']}",
                         "-D", f"CARGO_TARGET={plan['env']['CARGO_TARGET_DIR']}"]
            argv += ["-f", config["profile"], sys.executable, "-B", __file__, str(plan_path), stage, "--worker"]
            ledger["workerArgv"] = argv
            write(folder / "ledger.json", ledger)
            with (folder / "stdout.log").open("xb") as out, (folder / "stderr.log").open("xb") as err:
                child = subprocess.Popen(argv, cwd=plan["cwd"], env=env, stdout=out, stderr=err, start_new_session=True)
                next_disk = time.monotonic() + 4
                while child.poll() is None:
                    now = time.monotonic()
                    if now - started >= float(cap) - 14:
                        record.fail("deadline", "soft deadline")
                        break
                    members = monitor.monitor.members(child.pid)
                    rss = sum(value for _, value in members)
                    if now >= next_disk:
                        disk = historical + monitor.disk_bytes(mutable, timeout=30)
                        next_disk = time.monotonic() + 4
                    ledger["samples"].append([round(now - started, 6), len(members), rss, disk])
                    if len(members) > 24 or rss > monitor.RSS_LIMIT or disk > monitor.DISK_LIMIT:
                        record.fail("monitor", "resource budget")
                        break
                    time.sleep(.2)
        except BaseException as error:
            record.fail("monitor", repr(error))
        finally:
            for sig in handlers:
                signal.signal(sig, signal.SIG_IGN)
            def verify():
                check_pins(plan)
                if sha(plan_path) != ledger["matrixSHA256"]:
                    raise ValueError("command matrix changed during stage")
                verify_receipts(config["steps"], read(folder / "commands.json"), complete=True)
                if historical is not None:
                    ledger["finalDiskBytes"] = historical + monitor.disk_bytes(mutable, timeout=30)
                    if ledger["finalDiskBytes"] > monitor.DISK_LIMIT:
                        raise ValueError("disk budget after cleanup")
            try:
                result = record.finish(child, monitor.monitor.clean, verify, started, cap)
            finally:
                for sig, handler in handlers.items():
                    signal.signal(sig, handler)
        return result


if __name__ == "__main__":
    path = Path(sys.argv[1]).resolve()
    plan = read(path)
    stage = sys.argv[2]
    if len(sys.argv) == 4 and sys.argv[3] == "--worker":
        raise SystemExit(worker(plan, stage))
    raise SystemExit(run(path, plan, stage))
