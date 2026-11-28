"""Compare complete manifest sets and capture tracked/unignored Git input bytes."""
import argparse
from hashlib import sha256
import json
import os
from pathlib import Path
import re
import subprocess


def fingerprint(row):
    if isinstance(row, str):
        digest, kind = row, 'file'
    elif isinstance(row, dict):
        digest, kind = row.get('sha256'), row.get('kind', 'file')
    else:
        raise ValueError('manifest entry must be a digest or file descriptor')
    if not isinstance(digest, str) or not re.fullmatch('[0-9a-f]{64}', digest):
        raise ValueError('manifest entry requires a SHA-256 digest')
    if kind not in ('file', 'symlink'):
        raise ValueError('unsupported manifest file kind')
    return digest, kind


def compare(before, after):
    for manifest in (before, after):
        if not isinstance(manifest, dict):
            raise ValueError('manifest must map complete paths to file descriptors')
        for path, row in manifest.items():
            if not isinstance(path, str) or not path:
                raise ValueError('manifest paths must be nonempty strings')
            fingerprint(row)
    left, right = set(before), set(after)
    changed = [p for p in sorted(left & right) if fingerprint(before[p]) != fingerprint(after[p])]
    return {'beforeFiles': len(left), 'afterFiles': len(right), 'added': sorted(right - left),
            'removed': sorted(left - right), 'modified': changed,
            'unchanged': [p for p in sorted(left & right) if p not in set(changed)]}


def snapshot(root):
    root = Path(root).resolve()
    paths = subprocess.check_output(['git', '--no-optional-locks', '-C', str(root),
                                     'ls-files', '--cached', '--others', '--exclude-standard', '-z'])
    result = {}
    for name in sorted(set(os.fsdecode(p) for p in paths.split(b'\0') if p)):
        path = root / name
        if path.is_symlink():
            data, kind = os.fsencode(os.readlink(path)), 'symlink'
        elif path.is_file():
            data, kind = path.read_bytes(), 'file'
        elif not path.exists():
            continue
        else:
            raise ValueError('not a regular Git input: ' + name)
        result[name] = {'sha256': sha256(data).hexdigest(), 'bytes': len(data), 'kind': kind}
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_subparsers(dest='mode', required=True)
    capture = modes.add_parser('snapshot')
    capture.add_argument('root', type=Path)
    diff = modes.add_parser('compare')
    diff.add_argument('before', type=Path)
    diff.add_argument('after', type=Path)
    args = parser.parse_args()
    if args.mode == 'snapshot':
        result = snapshot(args.root)
    else:
        result = compare(json.loads(args.before.read_bytes()), json.loads(args.after.read_bytes()))
        result['manifestInputs'] = [{'path': str(p.resolve()), 'sha256': sha256(p.read_bytes()).hexdigest()}
                                    for p in (args.before, args.after)]
    print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
