#!/usr/bin/env bash
# Verify prepared inputs. This script does not install tools or download dependencies.
set -euo pipefail

require_version() {
  local expected=$1 actual
  shift
  if ! actual=$("$@"); then
    printf 'BLOCKED runner input: cannot run %s\n' "$*" >&2
    exit 2
  fi
  if [[ $actual != "$expected" ]]; then
    printf 'BLOCKED runner input: %s requires %s, found %s\n' "$*" "$expected" "$actual" >&2
    exit 2
  fi
  printf '%s: %s\n' "$*" "$actual"
}

require_version v22.23.2 node --version
require_version 12.0.2 npm --version
require_version 'Python 3.9.6' python3 --version
require_version 'Version 7.0.2' node node_modules/typescript/bin/tsc --version

rustc -Vv
cargo --version
cargo clippy --version
rustfmt --version
if ! rustup target list --installed | grep -Fx wasm32-unknown-unknown; then
  echo 'BLOCKED runner input: wasm32-unknown-unknown standard library' >&2
  exit 2
fi

# The package-free xtask still uses the root lock. Confirm locked manifest and selected package inputs.
if ! cargo tree --offline --locked -p vesta-bindings --target wasm32-unknown-unknown \
  --no-default-features -e normal,build; then
  echo 'BLOCKED runner input: selected Cargo dependencies are not available offline' >&2
  exit 2
fi
