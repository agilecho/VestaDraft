#!/usr/bin/env bash
# Verify prepared inputs without installing tools or downloading dependencies.
set -euo pipefail

python_exec=${VESTA_PYTHON:-python3}
if ! command -v "$python_exec" >/dev/null; then
  printf 'BLOCKED runner input: cannot run %s\n' "$python_exec" >&2
  exit 2
fi
exec "$python_exec" -B xtask/ci.py runner-inputs
