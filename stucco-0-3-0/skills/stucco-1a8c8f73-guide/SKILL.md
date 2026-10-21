---
name: "stucco-1a8c8f73-guide"
description: "Use stucco 0.3.0 for the tasks covered in its release guidance; consult cited findings, recipe verification status, and required features."
depends-on: "stucco==0.3.0"
---

# stucco 0.3.0

Stucco 0.3.0 provides themed server rendering, native display and status components, optional Axum integration, typed collections, and form workflows. These focused recipes use the frozen 0.3.0 evidence; consult each recorded compilation result and feature selection. Storage integration and browser enhancement still require application code beyond these examples.

Use this guidance only with the stated release. Check the project's Cargo.lock before applying it. A compiled recipe passed cargo check; runtime behavior has not been tested. Read each recipe's feature and dependency requirements.

## Guidance

- [Select features from the 0.3.0 manifest](references/findings/release-features.md) — Documented.
- [Deliver assets with rendered HTML](references/findings/page-assets.md) — Documented.
- [Finish router construction with stucco](references/findings/axum-setup.md) — Documented.
- [Keep fragment responses paired with a full-page fallback](references/findings/fragment-negotiation.md) — Documented.
- [Match table controls to backend capabilities](references/findings/collection-contract.md) — Documented.
- [Preserve values and explain validation failures](references/findings/form-validation.md) — Documented.
- [Configure CSRF and flash layers explicitly](references/findings/csrf-flash.md) — Documented.
- [Keep conflict recovery and return navigation explicit](references/findings/conflicts-return-context.md) — Documented.
- [Validate themes and choose scheme ownership](references/findings/theme-validation.md) — Documented.
- [Preserve rendering and application responsibility boundaries](references/findings/rendering-boundaries.md) — Documented.
- [Separate stored values from table presentation](references/findings/numeric-display-pattern.md) — Inferred from the cited sources.
- [Compose native status, navigation, progress and disclosures](references/findings/status-and-details.md) — Documented.

## Recipes

- [Export a static page with its assets](references/recipes/static-page.md) — Compiled.
- [Serve full pages and a named fragment](references/recipes/axum-fragments.md) — Compiled.
- [Derive columns and format stored cents](references/recipes/derived-table.md) — Compiled.
- [Render validation errors from returned form state](references/recipes/bound-validation.md) — Compiled.
- [Render an inline document with a server-selected scheme](references/recipes/inline-server-theme.md) — Compiled.
- [Render status and details with native components](references/recipes/status-details.md) — Compiled.

## Coverage limitations

- Collection is bounded to 48 UTF-8 evidence files, 48 KiB per file, and 320 KiB total; examples and API documentation are sampled, not exhaustive. Rendered documentation and external documentation sites are not crawled.
- Pinned repository archive: 1 link or special-file entries were rejected without following or extracting them.
- 2 distinct eligible source files were omitted by the evidence budget; 31 files selected.

## Provenance

[Release, evidence hashes, review status, and compilation records](references/provenance.json). Paths in that record are relative to its directory. Dependency lockfiles are included alongside the records.
