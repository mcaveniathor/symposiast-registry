---
name: "stucco-adc7ae09-guide"
description: "Use stucco 0.5.0 for the tasks covered in its release guidance; consult cited findings, recipe verification status, and required features."
depends-on: "stucco==0.5.0"
---

# stucco 0.5.0

Focused guidance for generating a static browser-game shell with Stucco 0.5.0: linked content-hashed assets, a fixed canvas mount, labelled controls, and an explicit theme. No Axum dependency or Stucco enhancement runtime is required by this recipe. The game loader and game simulation remain separate responsibilities. See the generated verification record for compilation status; runtime trials are documented separately.

Use this guidance only with the stated release. Check the project's Cargo.lock before applying it. A compiled recipe passed cargo check; runtime behavior has not been tested. Read each recipe's feature and dependency requirements.

## Guidance

- [Export linked assets with the generated page](references/findings/static-assets.md) — Documented.
- [Keep a fixed canvas mount in trusted application markup](references/findings/stable-canvas.md) — Inferred from the cited sources.
- [Choose an explicit theme for a shell without Stucco scripts](references/findings/script-free-shell.md) — Documented.
- [Enable only the component groups used by the shell](references/findings/minimal-features.md) — Documented.

## Recipes

- [Export a static canvas shell with content-hashed assets](references/recipes/static-canvas-shell.md) — Compiled.

## Documentation examples

60 examples inventoried. [Adaptations, supporting examples, and skipped examples with reasons](references/examples.md); [complete inventory and coverage data](references/examples.json). This accounts only for the collected evidence.

## Coverage limitations

- Collection is bounded to 48 UTF-8 evidence files, 48 KiB per file, and 320 KiB total; examples and API documentation are sampled, not exhaustive. Rendered documentation and external documentation sites are not crawled.
- Pinned repository archive: 1 link or special-file entries were rejected without following or extracting them.
- 5 distinct eligible source files were omitted by the evidence budget; 38 files selected.

## Provenance

[Release, evidence hashes, review status, and compilation records](references/provenance.json). Paths in that record are relative to its directory. Dependency lockfiles are included alongside the records.
