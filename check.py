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

# Every release is discoverable, and catalog paths cannot escape the registry.
catalog = json.loads((root / "catalog.json").read_text())
assert isinstance(catalog, list) and catalog, "catalog must be a nonempty array"
listed = set()
for item in catalog:
    entry = item["entry"]
    assert isinstance(entry, str) and entry and all(c.isascii() and (c.isalnum() or c in "-_") for c in entry), "invalid entry directory"
    assert entry not in listed, f"duplicate catalog entry: {entry}"
    listed.add(entry)
    assert isinstance(item["summary"], str) and item["summary"].strip()
    assert isinstance(item["tags"], list) and item["tags"] and all(isinstance(t, str) and t.strip() for t in item["tags"])
    tag = item["bundle_release"]
    assert tag is None or (isinstance(tag, str) and tag and all(c.isascii() and (c.isalnum() or c in "-_.") for c in tag)), "invalid bundle release tag"
    assert len(list((root / entry).glob("skills/*/references/provenance.json"))) == 1, f"{entry}: expected one release skill"
actual = {path.relative_to(root).parts[0] for path in records}
assert listed == actual, f"catalog differs from exported entries: missing={actual - listed}, extra={listed - actual}"
print(f"Validated catalog coverage for {len(listed)} entries.")
