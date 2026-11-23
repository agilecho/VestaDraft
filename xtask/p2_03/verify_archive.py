"""Recompute C12/G31 archives and actual bytes without running product code."""
import io
import json
from pathlib import Path
import re
import subprocess
import tarfile
from run_evidence import read, sha, segment_steps, verify_receipts

ROOT = Path("/Users/agilecho/code/VestaDraft")
OLD = ROOT / "output/vesta-p2-03/20261122-build-window"


def tar_entries(stream, prefix=""):
    with tarfile.open(fileobj=stream) as archive:
        result = {}
        for entry in archive.getmembers():
            if not entry.isfile():
                continue
            name = entry.name.removeprefix("./").removeprefix(prefix)
            if name in result:
                raise ValueError("duplicate archive member")
            import hashlib
            result[name] = hashlib.sha256(archive.extractfile(entry).read()).hexdigest()
        return result


def verify():
    report = {}
    closure = read(OLD / "closure.sha256.json")
    for filename, expected in closure.items():
        path = Path(filename)
        if not path.is_absolute():
            path = OLD / path
        if sha(path) != expected:
            raise ValueError("historical closure changed " + str(path))
    report["closureFiles"] = len(closure)
    for version, rev in [(11, "90d4a0e"), (12, "8512162")]:
        with (OLD / f"c{version}-source-{rev}.tar.gz").open("rb") as stream:
            archive = tar_entries(stream, "VestaDraft/")
        original = tar_entries(io.BytesIO(subprocess.check_output(["git", "-C", str(ROOT), "archive", rev])))
        if archive != original:
            raise ValueError("source archive differs from commit " + rev)
        report[f"C{version}SourceFiles"] = len(archive)
    import hashlib
    for name, args in [
        ("candidate.patch", ["--binary", "773d2b9", "8512162"]),
        ("documentation.patch", ["8512162", "cab6b34"]),
    ]:
        raw = subprocess.check_output(["git", "-C", str(ROOT), "diff", *args])
        if hashlib.sha256(raw).hexdigest() != sha(OLD / name):
            raise ValueError("patch differs from the recorded revisions")
    manifest = read(OLD / "c12-inputs-archive-manifest.json")
    with (OLD / "c12-inputs.tar.gz").open("rb") as stream:
        entries = tar_entries(stream)
    if entries != {name: row["sha256"] for name, row in manifest.items()}:
        raise ValueError("archived input bytes differ")
    pins = read(OLD / "c12/candidate-inputs.sha256.json")
    original_pins = {row["originalPath"]: row["sha256"] for row in manifest.values()}
    for path, expected in original_pins.items():
        if pins[path] != expected:
            raise ValueError("archive and input pins differ")
    external = {name: digest for name, digest in pins.items() if name not in original_pins}
    if len(external) != 7 or any(sha(name) != digest for name, digest in external.items()):
        raise ValueError("external interpreter/toolchain pins differ")
    report.update(archivedInputs=len(entries), totalOriginalPins=len(pins))
    compiler = []
    stage = OLD / "c12/runtime/c12"
    for filename in ["test-artifacts.stdout", "final-cli.stdout"]:
        for line in (stage / filename).read_text().splitlines():
            try:
                row = json.loads(line)
            except ValueError:
                continue
            if row.get("reason") == "compiler-artifact" and row.get("executable"):
                compiler.append(row)
    linked = []
    for path, digest in read(stage / "sealed/artifacts.json").items():
        path = Path(path)
        if path.parent.name != "sealed" or not (path.name.startswith("test-") or path.name == "vestacraft-cli"):
            continue
        is_test = path.name.startswith("test-")
        matches = [row for row in compiler if row["target"]["name"] == path.name.removeprefix("test-")
                   and row["profile"]["test"] == is_test]
        if not matches or sha(path) != digest or any(sha(row["executable"]) != digest for row in matches):
            raise ValueError("sealed executable differs from current Cargo product")
        linked.append(str(path))
    report["executablesLinkedToCargoAtVerification"] = len(linked)
    matrix_path = OLD / "g31-matrix.json"
    matrix = read(matrix_path)
    original = OLD / "runtime/g31"
    prior = read(original / "executed-commands.json")
    actual = read(original / "independent-tail/executed-commands.json")
    driver = OLD / "resume-graph.py"
    invocation = read(original / "g30/graph-discovery-and-persistence/continuation-invocations.json")
    binding = {
        "originalMatrixSHA256": sha(matrix_path), "stepRange": [9, 11],
        "replacements": [{
            "step": 9, "name": matrix["steps"][8]["name"], "originalArgv": matrix["steps"][8]["argv"],
            "newArgv": actual[0]["argv"], "driverIndex": 2, "driverSHA256": sha(driver),
            "scope": invocation["newInvocations"],
        }],
    }
    selected = segment_steps(matrix_path, binding, prior)
    verify_receipts(selected, actual, complete=True)
    report["g31Replacement"] = {"observationMadeAfterExecution": True, "preExecutionBindingExisted": False,
                                "explicitObservedReplacement": binding}
    rust = 0
    for step in matrix["steps"]:
        if "selectedTests" not in step:
            continue
        matches = re.findall(r"(?m)^test (\S+) \.\.\. (\S+)\s*$", (original / (step["name"] + ".stdout")).read_text())
        if sorted(name for name, _ in matches) != sorted(step["selectedTests"]) or any(r != "ok" for _, r in matches):
            raise ValueError("selected C12 test output differs")
        rust += len(matches)
    report["C12SelectedRustTests"] = rust
    base = original / "g30/native-grid-layer-thermal-view-and-fdat/artifact-only"
    projects = {}
    for stem in ["rich", "standalone", "source-history"]:
        pair = []
        for suffix in ["", "-continued"]:
            name = stem + suffix + ".vesta"
            primary = base / name
            manifest = read(base / (name + ".resources") / "manifests" / (sha(primary) + ".json"))
            if manifest["primarySha256"] != sha(primary):
                raise ValueError("primary does not match its manifest")
            for name, row in manifest["dependencies"].items():
                path = base / name
                if row["relativePath"] != name or sha(path) != row["sha256"] or str(path.stat().st_size) != row["byteLength"]:
                    raise ValueError("actual dependency differs")
            source_path = base / manifest["sourceDocumentRelativePath"]
            if sha(source_path) != manifest["sourceDocumentSha256"]:
                raise ValueError("actual source document differs")
            source = read(source_path)
            resources = {row["key"]: row for row in source["resources"]}
            spans = []
            for entity in source["tree"]["members"]["sourceEntities"]["items"]:
                for node in entity["members"]["sourceRecords"]["items"]:
                    row = node["members"]
                    resource = resources[row["sourceResourceId"]["resourceKey"]]
                    raw = (base / resource["relativeBlobPath"]).read_bytes()
                    offset = int(row["byteOffset"]["value"])
                    length = int(row["byteLength"]["value"])
                    if not 0 <= offset <= offset + length <= len(raw):
                        raise ValueError("sourceRecord range is invalid")
                    digest = hashlib.sha256(raw[offset:offset + length]).hexdigest()
                    if digest != row["sha256"]["value"] or hashlib.sha256(raw).hexdigest() != resource["sha256"]:
                        raise ValueError("sourceRecord actual bytes differ")
                    spans.append({"resourceId": resource["capturedResourceId"], "offset": offset, "length": length,
                                  "sha256": digest, "blobSHA256": resource["sha256"]})
            pair.append(spans)
        def normalized(rows):
            return [{key: value for key, value in row.items() if key != "resourceId"} for row in rows]
        if not pair[0] or normalized(pair[0]) != normalized(pair[1]):
            raise ValueError("source bytes changed across the saved pair")
        if {row["resourceId"] for row in pair[0]} & {row["resourceId"] for row in pair[1]}:
            raise ValueError("resource IDs were reused across lifetimes")
        projects[stem] = pair
    report["actualSourceSpans"] = projects
    report["sourceSpanCount"] = sum(len(spans) for pair in projects.values() for spans in pair)
    return report


if __name__ == "__main__":
    print(json.dumps(verify(), indent=2))
