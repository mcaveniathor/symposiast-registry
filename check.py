"""Validate the evidence and lockfiles shipped with every registry skill."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parent
records = sorted(root.glob("*/skills/*/references/provenance.json"))
assert records, "no provenance records found"
for path in records:
    data = json.loads(path.read_text())
    assert data["schema_version"] == 1
    review = data["review"]
    assert review is None or (review["approved"] and not review["issues"])
    assert data["sources"] and data["recipes"]
    checks = [(s["path"], s["sha256"]) for s in data["sources"]]
    for recipe in data["recipes"]:
        assert recipe["status"] == "compiled"
        checks.append((recipe["lockfile_path"], recipe["lockfile_sha256"]))
    for relative, digest in checks:
        candidate = path.parent / relative
        assert not candidate.is_symlink()
        assert candidate.resolve().is_relative_to(path.parent.resolve())
        assert hashlib.sha256(candidate.read_bytes()).hexdigest() == digest, candidate
print(f"Validated {len(records)} release provenance records and all referenced file hashes.")
