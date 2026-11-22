"""Offline engineering checks for the root Cargo workspace."""

import argparse
from collections import namedtuple
import hashlib
import io
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]
CARGO = (os.environ.get("CARGO", "cargo"),)
LOCKED = ("--offline", "--locked")
WASM = "wasm32-unknown-unknown"
BINDINGS = "vesta-bindings"
LAYER_MEMBERS = {
    "vesta-bindings": ("crates/bindings", set()),
    "vesta-source-derive": ("crates/source-derive", set()),
    "vesta-model": ("crates/model", {"vesta-bindings", "vesta-source-derive"}),
    "vesta-commands": ("crates/commands", {"vesta-model"}),
    "vesta-codecs": ("crates/codecs", {"vesta-bindings", "vesta-model", "vesta-source-derive"}),
    "vestacraft-cli": (
        "apps/vestacraft-cli",
        {"vesta-bindings", "vesta-model", "vesta-commands", "vesta-codecs"},
    ),
    "xtask": ("xtask", set()),
}
DEV_EDGE = ("vesta-codecs", "vesta-commands", None)
UI_DEPENDENCY = re.compile(
    r"^(?:egui|eframe|epaint|ecolor|emath|wgpu|winit|tauri|iced|gtk[34]?|"
    r"slint|bevy|yew|leptos|dioxus|web-sys|craft-ui)(?:$|[-_])"
)
# Admission for compiling the staged candidate belongs to P2-03.
PENDING_C8 = ("vesta-source-derive", "vesta-model", "vesta-commands", "vesta-codecs", "vestacraft-cli")
WASM_REQUIRED = (BINDINGS, "vesta-model", "vesta-commands")
TEST_TARGET = "generated_contracts"
TS_PACKAGE = "node_modules/typescript"
TS_COMPILER = f"{TS_PACKAGE}/bin/tsc"
BINDINGS_TESTS = {
    "generated_typed_bytes_and_rejections",
    "profiles_match_full_independent_vector_table",
}
BINDINGS_FIXTURES = (
    "tests/fixtures/resources.json",
    "contracts/generated/profile-input-manifest.json",
)
CODECS_FIXTURE_DIRS = (
    "output/vesta-p2-03/20261122-c8-dsh-followup/fixtures",
    "output/vesta-p2-03/20261122-c8-gsas-moment-intake/fixtures",
)
CODECS_COMPILE_FIXTURES = tuple(
    f"{directory}/{stem}.{suffix}"
    for directory, stem in zip(CODECS_FIXTURE_DIRS, ("authored-single", "authored"))
    for suffix in ("exp", "chart.json", "frame.json", "expected.json")
)
LOCKS = ("Cargo.lock", "package-lock.json")
CONTRACT_FILES = (
    *LOCKS,
    "contracts",
    "apps/vestacraft-web/src/kernel",
    "crates/bindings/src/generated.rs",
    "crates/bindings/src/profile_digests.rs",
    "crates/bindings/src/resource_roles.rs",
    "crates/model/src/source7/generated.rs",
    "crates/model/src/source7/coverage.json",
)
SNAPSHOT_INPUTS = (
    *LOCKS,
    ".cargo",
    "Cargo.toml",
    "package.json",
    "rust-toolchain.toml",
    "toolchain.inputs.json",
    "toolchain.sources.json",
    "tsconfig.contracts.json",
    "apps",
    "assets",
    "contracts",
    "crates",
    "scripts",
    "tests",
    "xtask",
)
Edge = namedtuple("Edge", "name kind target optional")


class Failed(Exception):
    """A check ran and found a problem."""


class Blocked(Exception):
    """A required input is unavailable."""


def run(argv, cwd=ROOT, capture=False, quiet=False, cargo_inputs=False, env=None):
    print(f"+ (cd {shlex.quote(str(cwd))} && {shlex.join(argv)})", flush=True)
    try:
        result = subprocess.run(
            argv, cwd=cwd, check=False, env=env,
            stdout=subprocess.PIPE if capture else None,
            stderr=subprocess.PIPE if cargo_inputs else None,
        )
    except OSError as error:
        raise Blocked(f"{argv[0]}: {error}") from error
    print(f"exit {result.returncode}", flush=True)
    if capture and not quiet:
        sys.stdout.buffer.write(result.stdout)
        sys.stdout.flush()
    if result.stderr:
        sys.stderr.buffer.write(result.stderr)
        sys.stderr.flush()
    if result.returncode:
        if cargo_inputs and re.search(
            rb"(?i)(offline mode|--offline was specified|failed to download)", result.stderr
        ):
            raise Blocked("locked Cargo inputs missing from the offline cache")
        raise Failed(f"{shlex.join(argv)} exited {result.returncode}")
    return result.stdout


def checked(name, action):
    try:
        action()
    except Blocked as error:
        print(f"BLOCKED {name}: {error}", flush=True)
        return 2
    except (Failed, OSError, ValueError) as error:
        print(f"FAILED {name}: {error}", flush=True)
        return 1
    print(f"PASS {name}", flush=True)
    return 0


def combined(results):
    return 1 if 1 in results else 2 if 2 in results else 0


def metadata():
    output = run(CARGO + ("metadata", *LOCKED, "--no-deps", "--format-version", "1"),
                 capture=True, quiet=True)
    data = json.loads(output)
    members = set(data["workspace_members"])
    return [package for package in data["packages"] if package["id"] in members]


def layers():
    packages = metadata()
    by_path = {Path(p["manifest_path"]).parent.resolve(): p["name"] for p in packages}
    errors = []
    for path, name in sorted(by_path.items()):
        if name not in LAYER_MEMBERS or path != ROOT / LAYER_MEMBERS[name][0]:
            errors.append(f"unclassified workspace member {name}: {path}")
    if errors:
        raise Failed("\n".join(errors))
    graph = {p["name"]: [] for p in packages}
    for package in packages:
        parent = package["name"]
        for dep in package["dependencies"]:
            dep_path = dep.get("path")
            if dep_path:
                name = by_path.get(Path(dep_path).resolve())
                if name is None:
                    errors.append(f"{parent}: unclassified path dependency {dep['name']} ({dep_path})")
                    continue
            elif UI_DEPENDENCY.match(dep["name"]):
                name = dep["name"]
            elif any(UI_DEPENDENCY.match(feature) for feature in dep["features"]):
                errors.append(f"{parent}: UI feature of {dep['name']} ({dep['features']})")
                continue
            else:
                continue
            edge = Edge(name, dep["kind"] or "normal", dep["target"], dep["optional"])
            graph[parent].append(edge)
            print(f"edge {parent} -> {describe(edge)}")

    for parent, edges in graph.items():
        for edge in edges:
            if not permits(parent, edge):
                errors.append(f"forbidden direct dependency: {parent} -> {describe(edge)}")
        # The first edge may be dev-only. Cargo does not propagate downstream dev dependencies.
        todo = [(edge, (describe(edge),), edge.kind == "dev") for edge in edges]
        seen = set()
        while todo:
            edge, chain, dev = todo.pop()
            key = (edge.name, dev)
            if key in seen:
                continue
            seen.add(key)
            if len(chain) > 1 and edge.name not in reachable_layers(parent, dev):
                errors.append(f"forbidden transitive dependency: {parent} -> {' -> '.join(chain)}")
            todo.extend((child, (*chain, describe(child)), dev)
                        for child in graph.get(edge.name, ()) if child.kind != "dev")
    if errors:
        raise Failed("\n".join(sorted(set(errors))))
    print(f"{len(packages)} workspace members; declared normal/build/dev edges include all cfg targets")


def describe(edge):
    optional = ", optional" if edge.optional else ""
    return f"{edge.name} [{edge.kind}, {edge.target or 'all targets'}{optional}]"


def permits(parent, edge):
    if (parent, edge.name, edge.target) == DEV_EDGE and edge.kind == "dev" and not edge.optional:
        return True
    return edge.kind in {"normal", "build", "dev"} and edge.name in LAYER_MEMBERS[parent][1]


def reachable_layers(parent, dev):
    reached = set()
    todo = list(LAYER_MEMBERS[parent][1])
    if dev and parent == DEV_EDGE[0]:
        todo.append(DEV_EDGE[1])
    while todo:
        name = todo.pop()
        if name not in reached:
            reached.add(name)
            todo.extend(LAYER_MEMBERS[name][1])
    return reached


def require_files(root, paths):
    missing = [path for path in paths if not (root / path).is_file()]
    if missing:
        raise Blocked("missing required files: " + ", ".join(missing))


def tree_bytes(root, paths):
    files = {}
    for rel in paths:
        path = root / rel
        if path.is_dir() and not path.is_symlink():
            for child in sorted(path.rglob("*")):
                if child.is_symlink():
                    raise Failed(f"unexpected symlink in protected inputs: {child}")
                if child.is_file():
                    files[child.relative_to(root).as_posix()] = child.read_bytes()
        elif path.is_file() and not path.is_symlink():
            files[rel] = path.read_bytes()
    return files


def assert_bytes(expected, actual, description):
    changed = sorted(name for name in expected.keys() | actual.keys()
                     if expected.get(name) != actual.get(name))
    if changed:
        raise Failed(f"{description}: " + ", ".join(changed))


def contracts():
    tool_version("node", "--version", "v22.23.2")
    python_version()
    packages = typescript_packages()
    archive = run(("git", "archive", "--format=tar", "HEAD", *CONTRACT_FILES),
                  capture=True, quiet=True)
    with tarfile.open(fileobj=io.BytesIO(archive)) as tree:
        expected = {item.name: tree.extractfile(item).read() for item in tree if item.isfile()}
    if sum(name.startswith("contracts/generated/cpj/") and name.endswith(".cpj") for name in expected) != 9:
        raise Failed("expected nine CPJ files in Git")
    assert_bytes(expected, tree_bytes(ROOT, CONTRACT_FILES), "protected bytes differ from HEAD")

    tracked = run(("git", "ls-files", "-z", "--", *SNAPSHOT_INPUTS),
                  capture=True, quiet=True).split(b"\0")
    with tempfile.TemporaryDirectory(prefix="vesta-contracts-") as temporary, \
            tempfile.TemporaryDirectory(prefix="vesta-contract-probes-") as scratch:
        root = Path(temporary).resolve()
        # Probe paths must be physical and outside the snapshot's Node module lookup tree.
        scratch = str(Path(scratch).resolve())
        child_env = dict(os.environ, TMPDIR=scratch, TMP=scratch, TEMP=scratch)
        print(f"contract snapshot: {root}", flush=True)
        for entry in set(tracked) - {b""}:
            rel = os.fsdecode(entry)
            source = ROOT / rel
            if source.is_symlink() or not source.is_file():
                raise Blocked(f"snapshot input is not a tracked regular file: {rel}")
            destination = root / rel
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
        for package in packages:
            shutil.copytree(ROOT / package, root / package)
        try:
            results = [
                checked("I33/NS2 contracts", lambda: run(
                    ("node", "scripts/test-contracts.mjs"), root, env=child_env)),
                checked("Source7 exact outputs", lambda: run(
                    (sys.executable, "-B", "scripts/generate-source7.py", "--check"), root)),
            ]
        finally:
            assert_bytes(expected, tree_bytes(root, CONTRACT_FILES), "contract snapshot changed")
            assert_bytes(expected, tree_bytes(ROOT, CONTRACT_FILES), "source checkout changed")
    if combined(results) == 1:
        raise Failed("a contract check failed")
    if combined(results) == 2:
        raise Blocked("a contract input is missing")
    print("all nine CPJ files and protected generated outputs match Git bytes")


def tool_version(tool, flag, expected):
    version = run((tool, flag), capture=True).decode().strip()
    if version != expected:
        raise Blocked(f"{tool} requires {expected}, found {version}")


def python_version():
    found = ".".join(map(str, sys.version_info[:3]))
    print(f"Python {found} ({sys.executable})", flush=True)
    if found != "3.9.6":
        raise Blocked(f"Python 3.9.6 required, found {found}")


def typescript_packages():
    require_files(ROOT, (TS_COMPILER, f"{TS_PACKAGE}/package.json"))
    host = run(("node", "-p", "process.platform + '-' + process.arch"), capture=True).decode().strip()
    platform_package = f"node_modules/@typescript/typescript-{host}"
    packages = (TS_PACKAGE, platform_package)
    require_files(ROOT, tuple(f"{path}/package.json" for path in packages))
    locked = json.loads((ROOT / "package-lock.json").read_text())["packages"]
    for path in packages:
        installed = json.loads((ROOT / path / "package.json").read_text())["version"]
        if installed != "7.0.2" or installed != locked.get(path, {}).get("version"):
            raise Blocked(f"missing locked TypeScript 7.0.2 package: {path}")
    version = run(("node", TS_COMPILER, "--version"), capture=True).decode().strip()
    if version != "Version 7.0.2":
        raise Blocked(f"TypeScript compiler version mismatch: {version}")
    return packages


def types():
    tool_version("node", "--version", "v22.23.2")
    tool_version("npm", "--version", "12.0.2")
    typescript_packages()
    run(("node", TS_COMPILER, "-p", "tsconfig.contracts.json", "--noEmit"))

def rust_version():
    info = run(("rustc", "-Vv"), capture=True).decode().splitlines()
    if "release: 1.99.1" not in info:
        raise Blocked("Rust 1.99.1 required")
    return next(line.removeprefix("host: ") for line in info if line.startswith("host: "))


def input_check(package, target, edges="normal,build"):
    # A selected-package tree establishes closure without downloading unrelated workspace platforms.
    run(CARGO + ("tree", *LOCKED, "-p", package, "--target", target,
                 "--no-default-features", "-e", edges), cargo_inputs=True)
    directory = run(("rustc", "--print", "target-libdir", "--target", target),
                    capture=True).decode().strip()
    if not any(Path(directory).glob("libstd-*.rlib")):
        raise Blocked(f"missing Rust standard library for {target}: {directory}")


def native_lint(package):
    if package == BINDINGS:
        require_files(ROOT, BINDINGS_FIXTURES)
    input_check(package, rust_version(), edges="normal,build,dev")
    run(CARGO + ("clippy", *LOCKED, "-p", package, "--all-targets", "--", "-D", "warnings"),
        cargo_inputs=True)


def native_test():
    require_files(ROOT, BINDINGS_FIXTURES)
    host = rust_version()
    input_check(BINDINGS, host, edges="normal,build,dev")
    output = run(CARGO + ("test", *LOCKED, "-p", BINDINGS, "--test", TEST_TARGET,
                          "--", "--test-threads=1"), capture=True, cargo_inputs=True)
    passed = re.findall(r"^test (.+) \.\.\. ok$", output.decode(), flags=re.MULTILINE)
    if len(passed) != len(BINDINGS_TESTS) or set(passed) != BINDINGS_TESTS:
        raise Failed(f"Rust test inventory mismatch: expected {sorted(BINDINGS_TESTS)}, passed {passed}")
    print(f"{len(passed)} required Rust tests passed")


def wasm_check(packages):
    host = rust_version()
    for package in packages:
        input_check(package, WASM)
    host_lib = run(("rustc", "--print", "target-libdir", "--target", host),
                   capture=True).decode().strip()
    if not any(Path(host_lib).glob("libstd-*.rlib")):
        raise Blocked(f"missing host Rust standard library for {host}")
    selection = tuple(arg for package in packages for arg in ("-p", package))
    output = run(CARGO + ("check", *LOCKED, "--target", WASM, "--lib",
                          "--no-default-features", "--message-format=json-render-diagnostics",
                          *selection), capture=True, cargo_inputs=True)
    ids = {p["id"]: p["name"] for p in metadata()}
    actual = {
        ids[event["package_id"]]
        for line in output.splitlines()
        for event in (json.loads(line),)
        if event.get("reason") == "compiler-artifact"
        and event["package_id"] in ids
        and "lib" in event["target"]["kind"]
        and not event["features"]
        and any(WASM in Path(path).parts for path in event["filenames"])
    }
    if set(packages) - actual:
        raise Failed(f"WASM artifacts did not cover required packages: {sorted(set(packages) - actual)}")
    print(f"{WASM} selected libraries: {', '.join(packages)} (no default features)")


def wait_for_c8(packages, fixtures=()):
    missing = [path for path in fixtures if not (ROOT / path).is_file()]
    details = ("; missing compile-time fixtures: " + ", ".join(missing)) if missing else ""
    raise Blocked(f"P2-03 C8 candidate admission pending for {', '.join(packages)}{details}")


def wasm(independent):
    selection = (BINDINGS,) if independent else WASM_REQUIRED
    admitted = tuple(package for package in selection if package not in PENDING_C8)
    pending = tuple(package for package in selection if package in PENDING_C8)
    results = [checked("WASM/" + "+".join(admitted), lambda: wasm_check(admitted))]
    if pending:
        results.append(checked("WASM shared kernel", lambda: wait_for_c8(pending)))
    return combined(results)


def ci(independent):
    scope = "independent" if independent else "full"
    print(f"VestaDraft CI scope: {scope}", flush=True)
    require_files(ROOT, LOCKS)
    locked = tree_bytes(ROOT, LOCKS)
    print("locked inputs: " + ", ".join(
        f"{name} SHA-256 {hashlib.sha256(data).hexdigest()}" for name, data in locked.items()), flush=True)
    results = []
    fmt_scope = ("-p", "xtask") if independent else ("--all",)
    try:
        checks = (
            ("layers", layers),
            ("contracts", contracts),
            ("TypeScript", types),
            ("fmt/xtask" if independent else "fmt/workspace",
             lambda: run(CARGO + ("fmt", *fmt_scope, "--", "--check"))),
            ("Clippy/xtask", lambda: native_lint("xtask")),
            ("Clippy/bindings", lambda: native_lint(BINDINGS)),
            ("Rust tests/bindings", native_test),
        )
        results.extend(checked(name, action) for name, action in checks)
        if not independent:
            results.append(checked("Clippy and Rust tests/C8",
                                   lambda: wait_for_c8(PENDING_C8, CODECS_COMPILE_FIXTURES)))
        results.append(wasm(independent))
    finally:
        results.append(checked("lockfiles byte stability",
                               lambda: assert_bytes(locked, tree_bytes(ROOT, LOCKS), "lockfiles changed")))
    outcome = combined(results)
    print(f"CI {scope} conclusion: {('PASS', 'FAILED', 'BLOCKED')[outcome]}", flush=True)
    return outcome


def main():
    parser = argparse.ArgumentParser(prog="cargo xtask")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("contracts-check")
    sub.add_parser("layers")
    sub.add_parser("wasm")
    sub.add_parser("ci").add_argument("--independent", action="store_true")
    args = parser.parse_args()
    os.environ["CARGO_NET_OFFLINE"] = "true"
    os.environ["CARGO_BUILD_JOBS"] = "2"
    os.environ["CARGO_TERM_COLOR"] = "never"
    os.environ["VESTA_PYTHON"] = sys.executable
    if args.command == "ci":
        try:
            return ci(args.independent)
        except Blocked as error:
            print(f"BLOCKED CI: {error}", flush=True)
            return 2
    if args.command == "wasm":
        return wasm(False)
    return checked(args.command, contracts if args.command == "contracts-check" else layers)


if __name__ == "__main__":
    sys.exit(main())
