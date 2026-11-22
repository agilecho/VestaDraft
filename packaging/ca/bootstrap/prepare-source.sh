#!/usr/bin/env bash
set -euo pipefail

if [[ $# != 2 || -z $1 || -z $2 ]]; then
    printf 'Usage: %s <local-craft-anything-repo> <new-source-directory>\n' "$0" >&2
    exit 2
fi

upstream_repo=$1
destination=$2
upstream_commit=1f8508a104134c436cbba5ea2f675619ef47702f
bootstrap_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)

export GIT_NO_LAZY_FETCH=1 GIT_TERMINAL_PROMPT=0
if ! git -C "$upstream_repo" cat-file -e "$upstream_commit^{commit}" 2>/dev/null; then
    printf 'Missing local Craft Anything commit %s in %s\n' "$upstream_commit" "$upstream_repo" >&2
    exit 1
fi

mkdir -- "$destination"
trap 'rm -rf -- "$destination"' EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

git -C "$upstream_repo" archive --format=tar "$upstream_commit" | tar -xf - -C "$destination"
cp -- "$bootstrap_dir/Cargo.lock" "$destination/Cargo.lock"
cp -R -- "$bootstrap_dir/p1-ca-bootstrap" "$destination/crates/p1-ca-bootstrap"

trap - EXIT INT TERM
printf 'Prepared bootstrap source in %s\n' "$destination"
