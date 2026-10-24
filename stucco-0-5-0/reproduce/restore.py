"""Reconstruct a collected bundle from this entry's frozen evidence."""
import hashlib
import json
from pathlib import Path
import shutil
import sys

entry = Path(__file__).resolve().parents[1]
refs = entry / 'skills/stucco-adc7ae09-guide/references'
provenance = json.loads((refs / 'provenance.json').read_text())
destination = Path(sys.argv[1])
destination.mkdir(parents=True, exist_ok=False)
(destination / 'sources').mkdir()
sources = []
for source in provenance['sources']:
    raw = (refs / source['path']).read_bytes()
    if hashlib.sha256(raw).hexdigest() != source['sha256']:
        raise ValueError(f"Evidence checksum mismatch: {source['id']}")
    copied = dict(source)
    copied['path'] = f"sources/{source['id']}.txt"
    (destination / copied['path']).write_bytes(raw)
    sources.append(copied)
bundle = {
    'schema_version': 1,
    'release': provenance['release'],
    'sources': sources,
    'example_inventory': provenance['example_inventory'],
    'analysis': None,
    'review': None,
    'verification': [],
    'warnings': provenance['warnings'],
}
(destination / 'bundle.json').write_text(json.dumps(bundle, indent=2) + '\n')
shutil.copyfile(entry / 'reproduce/analysis.json', destination / 'analysis.json')
print(destination)
