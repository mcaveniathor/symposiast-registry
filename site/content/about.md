# A registry with its evidence attached

Symposiast Registry is a community collection of release-specific Rust crate guidance for [Symposium](https://symposium.dev/). [Symposiast](https://github.com/mcaveniathor/symposiast) collects published source and pinned repository evidence, produces practical findings and recipes, and verifies consumer examples before exporting portable skills.

## What an entry contains

- One exact crates.io release, with an explicit dependency predicate.
- Findings that distinguish documented guidance from inferred patterns.
- Complete recipes with required features and dependencies.
- Frozen evidence and retained upstream license notices.
- A provenance record with release metadata, source URLs and hashes, review status, compilation fingerprints, and recorded dependency lockfiles.

The site reads the registry's catalog and provenance at build time. New entries appear after their pull requests are accepted and the Pages workflow deploys. It is a curated collection, not a mirror of crates.io.

## What verification means

**Compiled** means the recorded recipe inputs passed Cargo checks with the recorded toolchain and lockfile. It does not establish runtime behavior, compatibility with every target, or recommendation quality. The current source bundle schema does not record the verification platform.

**Agent review approved** means the provenance contains an approved review with no unresolved issues. **No recorded agent review** means that record is absent; this includes the imported Stucco pilot. Neither label represents endorsement by the upstream crate maintainers.

The registry checker validates stored evidence and lockfile hashes. Plugin validation checks the portable manifests. Human review must still consider the actual guidance, source identity, collection limitations, and licenses.

## Current coverage

The initial entry covers Stucco 0.2.1: semantic static pages with hashed assets and Axum routes for HTML and stylesheets. Its analysis was imported, and both recipes have compilation records. Upstream MIT and Apache-2.0 license texts accompany the frozen evidence.

Other Symposiast examples are not automatically registry entries. Additional crates and releases need a submitted export and review. [Request the coverage you need](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml), or [contribute an entry](contributing.html).

## Corrections and releases

Different crate releases remain separate entries. Corrections to an existing crate release update its registry entry and receive a new bundle revision, preserving old downloadable archives. Consumers should inspect the exact resolved dependency before using guidance.

[Read the contribution and review process](contributing.html), [browse the source repository](https://github.com/mcaveniathor/symposiast-registry), or [report a correction](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=correction.yml).
