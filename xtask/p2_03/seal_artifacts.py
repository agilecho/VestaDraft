"""Copy exact Cargo outputs after a no-run compilation."""
from pathlib import Path
import json
import shutil
import sys
from run_evidence import read, sha, write

folder = Path(sys.argv[1])
sealed = folder / "sealed"
sealed.mkdir(exist_ok=False)
links = []
for filename in ["test-artifacts.stdout", "final-cli.stdout"]:
    for line in (folder / filename).read_text().splitlines():
        try:
            row = json.loads(line)
        except ValueError:
            continue
        if row.get("reason") != "compiler-artifact" or not row.get("executable"):
            continue
        source = Path(row["executable"])
        name = ("test-" if row["profile"]["test"] else "") + row["target"]["name"]
        target = sealed / name
        if target.exists():
            if sha(source) != sha(target):
                raise ValueError("ambiguous Cargo executable " + name)
            continue
        shutil.copy2(source, target)
        if sha(source) != sha(target):
            raise ValueError("artifact copy differs")
        links.append({"target": name, "cargoPath": str(source),
                      "sealedPath": str(target), "sha256": sha(target)})
if not any(row["target"] == "vestacraft-cli" for row in links):
    raise ValueError("CLI artifact was not produced")
write(sealed / "artifacts.json", links)
print(json.dumps({"sealedExecutables": len(links)}, indent=2))
