"""Duplicate Cargo reports must bind identical CLI bytes."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


class SealTests(unittest.TestCase):
    def test_cli_repeated_by_test_compilation_and_build_is_deduplicated(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            binary = root / "cli"
            binary.write_bytes(b"tool-only artifact bytes")
            row = {"reason": "compiler-artifact", "executable": str(binary),
                   "target": {"name": "vestacraft-cli"}, "profile": {"test": False}}
            for name in ["test-artifacts.stdout", "final-cli.stdout"]:
                (root / name).write_text(json.dumps(row) + "\n")
            result = subprocess.run([sys.executable, "-B", str(Path(__file__).with_name("seal_artifacts.py")), str(root)],
                                    capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr.decode())
            artifacts = json.loads((root / "sealed/artifacts.json").read_text())
            self.assertEqual(len(artifacts), 1)
            self.assertEqual((root / "sealed/vestacraft-cli").read_bytes(), binary.read_bytes())


if __name__ == "__main__":
    unittest.main()
