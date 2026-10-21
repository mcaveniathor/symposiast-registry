---
name: "stucco-0a431c79-guide"
description: "Use stucco 0.2.1 for the tasks covered in its release guidance; consult cited findings, recipe verification status, and required features."
depends-on: "stucco==0.2.1"
---

# stucco 0.2.1

Practical Stucco 0.2.1 guidance for semantic static pages, escaped content, deployment-aware hashed assets, and optional Axum integration. This bundle targets the published release at commit 303ee2433ce3c05d0af9c4b79a4e09958d374f92. Its two recipes cover static output and in-memory HTML/asset requests; they do not cover the full component catalog, browser behavior, or APIs added on repository HEAD after that release.

Use this guidance only with the stated release. Check the project's Cargo.lock before applying it. A compiled recipe passed cargo check; runtime behavior has not been tested. Read each recipe's feature and dependency requirements.

## Guidance

- [Use Page::main for the main landmark and skip link](references/findings/semantic-pages.md) — Documented.
- [Keep untrusted content in escaped text and checked URLs](references/findings/safe-text-and-links.md) — Documented.
- [Write hashed assets and match the deployment URL prefix](references/findings/static-assets-and-base-path.md) — Documented.
- [Enable axum explicitly and apply Stucco after the routes](references/findings/axum-integration.md) — Documented.

## Recipes

- [Export a static page with hashed assets](references/recipes/static-page.md) — Compiled.
- [Serve a document and its stylesheet through Axum](references/recipes/axum-page.md) — Compiled.

## Coverage limitations

- Collection is bounded to 48 UTF-8 evidence files, 48 KiB per file, and 320 KiB total; examples and API documentation are sampled, not exhaustive. Rendered documentation and external documentation sites are not crawled.
- Pinned repository archive: 1 link or special-file entries were rejected without following or extracting them.
- 2 distinct eligible source files were omitted by the evidence budget; 27 files selected.
- The published facade and pinned repository selection were supplemented with stucco-core/src/bundle.rs from the same release commit to document asset prefixes and export APIs. This is curated evidence, not exhaustive coverage of companion crates.
- Upstream MIT and Apache-2.0 license texts from the release commit are included as frozen sources so they accompany installed evidence. Compiler logs normalize the temporary build-directory prefix; recorded check results and lockfile hashes are unchanged.

## Provenance

[Release, evidence hashes, review status, and compilation records](references/provenance.json). Paths in that record are relative to its directory. Dependency lockfiles are included alongside the records.
