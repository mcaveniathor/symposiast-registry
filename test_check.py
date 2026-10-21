"""Exercise registry validation against independent, temporary entry mutations."""
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent


class RegistryValidation(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        shutil.copy2(ROOT / "check.py", self.root)
        shutil.copy2(ROOT / "catalog.json", self.root)
        self.catalog = json.loads((self.root / "catalog.json").read_text())
        for entry in self.catalog:
            shutil.copytree(ROOT / entry["entry"], self.root / entry["entry"])

    def check(self, succeeds):
        (self.root / "catalog.json").write_text(json.dumps(self.catalog))
        result = subprocess.run(["python3", str(self.root / "check.py")], capture_output=True, text=True)
        self.assertEqual(result.returncode == 0, succeeds, result.stdout + result.stderr)

    def test_published_catalog(self):
        self.check(True)

    def test_duplicate_entry(self):
        self.catalog.append(self.catalog[0].copy())
        self.check(False)

    def test_unlisted_release(self):
        original = self.catalog[0]["entry"]
        shutil.copytree(self.root / original, self.root / "unlisted-release")
        self.check(False)

    def test_missing_entry(self):
        self.catalog[0]["entry"] = "missing-release"
        self.check(False)

    def test_path_escape(self):
        self.catalog[0]["entry"] = "../outside"
        self.check(False)

    def test_unpublished_bundle(self):
        self.catalog[0]["bundle_release"] = None
        self.check(True)

    def test_invalid_release_link(self):
        self.catalog[0]["bundle_release"] = "../../invalid"
        self.check(False)

    def test_tampered_evidence(self):
        record = next(self.root.glob("*/skills/*/references/provenance.json"))
        source = json.loads(record.read_text())["sources"][0]["path"]
        (record.parent / source).write_text("altered evidence")
        self.check(False)


if __name__ == "__main__":
    unittest.main()
