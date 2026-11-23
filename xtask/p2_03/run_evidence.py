"""Run evidence used by the P2-03 controlled build and test stages."""
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import time


def read(path):
    return json.loads(Path(path).read_bytes())


def sha(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def write(path, value):
    temporary = path.with_suffix(".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n")
    temporary.replace(path)


class FinishRecord:
    def __init__(self, path, ledger):
        self.path = path
        self.ledger = ledger
        ledger.update(firstFailure=None, laterFailures=[])

    def fail(self, phase, detail):
        failure = {"phase": phase, "detail": detail}
        if self.ledger["firstFailure"] is None:
            self.ledger["firstFailure"] = failure
        else:
            self.ledger["laterFailures"].append(failure)

    def finish(self, child, clean, verify, started, cap, clock=time.monotonic):
        ledger = self.ledger
        before = child.poll() if child is not None else None
        ledger.update(state="finishing", childPid=child.pid if child is not None else None,
                      childReturnCodeBeforeCleanup=before)
        if not ledger["firstFailure"]:
            if before is None:
                self.fail("supervisor", "no worker exit observed before cleanup")
            elif before != 0:
                self.fail("child", f"worker return code {before}")
        try:
            write(self.path, ledger)
        finally:
            if child is not None:
                try:
                    if not clean(child):
                        self.fail("cleanup", "owned process group still present")
                except Exception as error:
                    self.fail("cleanup", repr(error))
                finally:
                    code = child.poll()
                    ledger.update(childReturnCode=code, childSignal=-code if code is not None and code < 0 else None)
            else:
                ledger.update(childReturnCode=None, childSignal=None)
        write(self.path, ledger)
        try:
            verify()
        except Exception as error:
            self.fail("verification", repr(error))
        elapsed = clock() - started
        if Decimal(str(elapsed)) > cap:
            self.fail("deadline", "hard deadline exceeded")
        first = ledger["firstFailure"]
        code = before
        if (first is not None and first["phase"] != "child") or ledger["laterFailures"]:
            code = 124
        elif before is not None and before < 0:
            code = 128 - before
        ledger.update(state="finished", seconds=f"{elapsed:.9f}", exit=code)
        write(self.path, ledger)
        return code


def verify_receipts(expected, actual, *, complete):
    if len(actual) > len(expected) or complete and (not actual or len(actual) != len(expected)):
        raise ValueError("command receipt count differs from the matrix")
    for step, receipt in zip(expected, actual):
        if (step["name"], step["argv"]) != (receipt["name"], receipt["argv"]):
            raise ValueError(f"command name or argv differs at {step['name']}")
        if complete and (receipt["exit"] != 0 or receipt.get("coverageError") is not None):
            raise ValueError(f"command failed: {step['name']}")


def segment_steps(matrix, binding, prior):
    if sha(matrix) != binding["originalMatrixSHA256"]:
        raise ValueError("original command matrix changed")
    steps = read(matrix)["steps"]
    first, last = binding["stepRange"]
    if first != len(prior) + 1 or not 1 <= first <= last <= len(steps):
        raise ValueError("continuation step range overlaps or leaves a gap")
    if prior:
        verify_receipts(steps[:first - 1], prior, complete=True)
    selected = [dict(step) for step in steps[first - 1:last]]
    replaced = set()
    for replacement in binding["replacements"]:
        index = replacement["step"]
        if not first <= index <= last or index in replaced:
            raise ValueError("duplicate or out-of-range command replacement")
        replaced.add(index)
        step = selected[index - first]
        if (step["name"], step["argv"]) != (replacement["name"], replacement["originalArgv"]):
            raise ValueError("replacement's original name or argv does not match")
        argv = replacement["newArgv"]
        driver_index = replacement["driverIndex"]
        scope = replacement["scope"]
        if not scope or not isinstance(scope, list) or not all(isinstance(item, str) and item for item in scope):
            raise ValueError("replacement has no explicit execution scope")
        if not 0 <= driver_index < len(argv):
            raise ValueError("replacement driver is missing from argv")
        driver = Path(argv[driver_index])
        if not driver.is_absolute() or not driver.is_file() or sha(driver) != replacement["driverSHA256"]:
            raise ValueError("replacement driver path or SHA-256 differs")
        step["argv"] = argv
    return selected


def build_limit(window, before, requested):
    if (window["windowSeconds"] != 1783 or window["hardBuildInvocationCap"] is not None
            or "maxAdditionalBuildGroups" in window):
        raise ValueError("incremental build authorization fields conflict")
    cap = Decimal(requested)
    available = [
        Decimal(window["windowSeconds"]) - Decimal(window["usedSeconds"]),
        Decimal(14400) - Decimal(before["card"]),
        Decimal(10800) - Decimal(before["continuation"]),
        Decimal(3000) - Decimal(before["package"]),
    ]
    if any(not value.is_finite() for value in [cap, *available]) or not 0 < cap <= min(available):
        raise ValueError("incremental build exceeds the remaining time budget")
    return cap
