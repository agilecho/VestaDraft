"""Tool-only regression checks. All children are plain Python processes."""
from decimal import Decimal
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import unittest
from run_evidence import FinishRecord, build_limit, read, segment_steps, sha, verify_receipts


class EvidenceTests(unittest.TestCase):
    def test_monitor_failure_survives_cleanup_verification_and_deadline(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "ledger.json"
            child = subprocess.Popen([sys.executable, "-c", "import time;time.sleep(10)"],
                                     start_new_session=True)
            record = FinishRecord(path, {})
            record.fail("monitor", "disk timeout")
            def clean(process):
                self.assertEqual(read(path)["firstFailure"]["detail"], "disk timeout")
                self.assertIsNone(read(path)["childReturnCodeBeforeCleanup"])
                os.killpg(process.pid, signal.SIGTERM)
                process.wait(timeout=3)
                raise ValueError("cleanup report")
            def verify():
                self.assertEqual(read(path)["childSignal"], signal.SIGTERM)
                raise ValueError("incomplete commands")
            self.assertEqual(record.finish(child, clean, verify, 0, 1, lambda: 2), 124)
            result = read(path)
            self.assertEqual(result["firstFailure"]["phase"], "monitor")
            self.assertEqual([row["phase"] for row in result["laterFailures"]],
                             ["cleanup", "verification", "deadline"])
            self.assertEqual(result["childReturnCode"], -signal.SIGTERM)

    def test_completed_child_is_recorded_before_final_verification(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "ledger.json"
            with subprocess.Popen([sys.executable, "-c", "raise SystemExit(101)"]) as child:
                child.wait(timeout=3)
                def verify():
                    self.assertEqual(read(path)["childReturnCodeBeforeCleanup"], 101)
                    self.assertEqual(read(path)["firstFailure"]["phase"], "child")
                result = FinishRecord(path, {}).finish(child, lambda _: True, verify, 0, 3, lambda: 1)
                self.assertEqual(result, 101)

    def test_signaled_child_keeps_its_signal(self):
        with tempfile.TemporaryDirectory() as directory:
            with subprocess.Popen([sys.executable, "-c", "import os,signal;os.kill(os.getpid(),signal.SIGTERM)"]) as child:
                child.wait(timeout=3)
                record = FinishRecord(Path(directory) / "ledger.json", {})
                self.assertEqual(record.finish(child, lambda _: True, lambda: None, 0, 3, lambda: 1), 143)
                self.assertEqual(record.ledger["childSignal"], signal.SIGTERM)

    def test_replacement_binds_full_argv_driver_and_scope(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            driver = root / "resume.py"
            driver.write_text("# tool fixture\n")
            matrix = root / "matrix.json"
            steps = [{"name": str(i), "argv": ["old", str(i)]} for i in range(11)]
            matrix.write_text(json.dumps({"steps": steps}))
            new = [sys.executable, "-B", str(driver)]
            binding = {"originalMatrixSHA256": sha(matrix), "stepRange": [9, 11],
                       "replacements": [{"step": 9, "name": "8", "originalArgv": steps[8]["argv"],
                                         "newArgv": new, "driverIndex": 2,
                                         "driverSHA256": sha(driver), "scope": ["unstarted-reopen"]}]}
            selected = segment_steps(matrix, binding, [dict(s, exit=0) for s in steps[:8]])
            self.assertEqual(selected[0]["argv"], new)
            receipts = [dict(row, exit=0) for row in selected]
            verify_receipts(selected, receipts, complete=True)
            receipts[-1]["argv"] = ["unbound"]
            with self.assertRaisesRegex(ValueError, "argv"):
                verify_receipts(selected, receipts, complete=True)
            driver.write_text("# modified\n")
            with self.assertRaisesRegex(ValueError, "driver"):
                segment_steps(matrix, binding, [dict(s, exit=0) for s in steps[:8]])

    def test_missing_receipts_and_empty_selection_cannot_pass(self):
        expected = [{"name": "one", "argv": ["python"]}]
        with self.assertRaisesRegex(ValueError, "count"):
            verify_receipts(expected, [], complete=True)
        with self.assertRaisesRegex(ValueError, "count"):
            verify_receipts([], [], complete=True)
        with self.assertRaisesRegex(ValueError, "failed"):
            verify_receipts(expected, [dict(expected[0], exit=1)], complete=True)

    def test_budget_and_conflicting_authorization(self):
        window = {"windowSeconds": 1783, "hardBuildInvocationCap": None, "usedSeconds": "19.823152167"}
        before = {"card": "1308.876137004", "continuation": "1170.848808837", "package": "651.699882710"}
        self.assertEqual(build_limit(window, before, "1763.176847833"), Decimal("1763.176847833"))
        with self.assertRaisesRegex(ValueError, "budget"):
            build_limit(window, before, "1763.176847834")
        with self.assertRaisesRegex(ValueError, "authorization"):
            build_limit(dict(window, maxAdditionalBuildGroups=1), before, "60")


if __name__ == "__main__":
    unittest.main()
