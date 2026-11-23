"""Exercise the real supervisor with a plain Python child and injected failure."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from run_evidence import read, sha, write


class SupervisorTests(unittest.TestCase):
    def test_actual_entry_records_nonzero_worker_and_does_not_overwrite_cause(self):
        source = Path(__file__).resolve().parent
        repo = Path("/Users/agilecho/code/VestaDraft")
        monitor = repo / "output/vesta-p2-03/20261121-g26-remainder/supervise.py"
        clarification = repo / "output/vesta-execution/20261122-p2-03-c12-g31-dsh-recheck/authorization-clarification.json"
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            lock = root / "group.lock"
            lock.touch()
            profile = root / "offline.sb"
            profile.write_text('(version 1)\n(allow default)\n(deny network*)\n(deny file-write*)\n(allow file-write* (subpath (param "STAGE_ROOT")))\n')
            inherited = root / "inherited.json"
            write(inherited, {"cumulative": {"cardSeconds": "1308.876137004",
                  "continuationSeconds": "1170.848808837", "testGroups": {"used": 31},
                  "builds": {"windowSecondsUsed": "19.823152167"}}, "packageSecondsUsed": "651.699882710"})
            auth = root / "authorization.json"
            write(auth, {"approved": True, "candidateSHA256": "tool-fixture", "stage": "tool",
                         "capSeconds": 30, "windowClarification": str(clarification),
                         "windowClarificationSHA256": sha(clarification)})
            pins = root / "pins.json"
            write(pins, {str(source / name): sha(source / name) for name in ["supervise.py", "run_evidence.py"]})
            plan = root / "matrix.json"
            write(plan, {"candidateSHA256": "tool-fixture", "output": str(root / "runtime"),
                  "cwd": str(root), "pins": str(pins), "monitor": str(monitor), "p2Root": str(root),
                  "lock": str(lock), "inheritedBudget": str(inherited), "mutableRoots": [str(root / "runtime")],
                  "env": {"RUSTC": sys.executable, "CARGO_HOME": str(root), "CARGO_TARGET_DIR": str(root)},
                  "stages": {"tool": {"class": "build", "authorization": str(auth),
                    "authorizationSHA256": sha(auth), "capSeconds": 30, "profile": str(profile),
                    "steps": [{"name": "plain-python", "argv": [sys.executable, "-c", "raise SystemExit(7)"]}]}}})
            result = subprocess.run([sys.executable, "-B", str(source / "supervise.py"), str(plan), "tool"],
                                    capture_output=True, timeout=30)
            self.assertEqual(result.returncode, 124, result.stderr.decode())
            row = read(root / "runtime/tool/ledger.json")
            self.assertEqual(row["firstFailure"], {"phase": "child", "detail": "worker return code 7"})
            self.assertEqual(row["childReturnCode"], 7)
            self.assertEqual(row["laterFailures"][0]["phase"], "verification")
            self.assertEqual(row["workerArgv"][-3:], [str(plan), "tool", "--worker"])


if __name__ == "__main__":
    unittest.main()
