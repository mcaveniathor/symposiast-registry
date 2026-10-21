# Symposiast registry

Release-specific Rust crate guidance for [Symposium](https://symposium.dev/), generated and validated with [Symposiast](https://github.com/mcaveniathor/symposiast).

Browse the [registry website](https://mcaveniathor.github.io/symposiast-registry/), [submit an entry](CONTRIBUTING.md#submit-verified-guidance), or [request a crate/release](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml).

## Catalog

| Crate | Release | Recipes | Verification | Guidance |
| --- | --- | --- | --- | --- |
| Stucco | 0.2.1 | Static page and assets; Axum HTML and stylesheet routes | Both recipes compiled; imported analysis | [Read the skill](stucco-0-2-1/skills/stucco-0a431c79-guide/SKILL.md) |

[Download the full Stucco bundle](https://github.com/mcaveniathor/symposiast-registry/releases/tag/stucco-0.2.1-r1) for runnable Cargo projects, compiler logs, lockfiles, frozen sources, and local MCP installation.

## Subscribe

Configure your agents with Symposium, then add this entry to your existing `~/.symposium/config.toml` (or the configuration under `SYMPOSIUM_HOME`):

```toml
[[registry]]
name = "symposiast"
git = "https://github.com/mcaveniathor/symposiast-registry"
```

Inside your Rust workspace:

```sh
cargo agents sync
cargo agents status
```

The Stucco skill matches `stucco==0.2.1`. Each release's skill has a distinct identity. Unsupported releases do not receive another release's guidance. This registry supplies skills and their references; it does not register hooks or MCP servers or require the Symposiast executable.

Symposium 0.5.1 has been checked with a relocated registry and isolated Claude Code and Codex configurations, including repeated sync and removal after a version mismatch. Confirm the resolved version and source in your own workspace before applying a recipe. Avoid installing both this entry and the full runtime plugin for the same release in one workspace, since they share the skill identity.

## Evidence and verification

The skill includes all frozen evidence and a [provenance record](stucco-0-2-1/skills/stucco-0a431c79-guide/references/provenance.json), including source URLs and hashes, original release metadata, compilation fingerprints, and dependency lockfiles. Paths in the record are relative to its directory. Compilation establishes that the recorded recipe inputs passed Cargo checks; it does not establish runtime behavior. The original bundle schema does not record the verification platform.

The Stucco analysis was imported and has no recorded agent review. Its executable assertions were checked separately in the [source project](https://github.com/mcaveniathor/symposiast/blob/main/plugins/stucco/README.md). This is third-party guidance, not a Stucco-maintainer endorsement.

## Submit guidance, request coverage, or report a correction

Read [CONTRIBUTING.md](CONTRIBUTING.md) for the complete generation, verification, export, pull-request, and maintainer review process.

- **Have guidance to share?** Fork the registry, export a verified bundle into a new entry directory, update `catalog.json`, run the checks, and submit a pull request.
- **Need a crate or release?** Search [existing issues](https://github.com/mcaveniathor/symposiast-registry/issues), then [request guidance](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml). Include the package, version, tasks, and feature/platform requirements. No bundle is needed to make a request.
- **Found incorrect guidance?** [Report a correction](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=correction.yml) with the entry, recipe/finding, and reproduction.

Different crate releases stay in separate directories. Corrections get a new registry commit and bundle revision; published archives are not replaced. Requests do not guarantee coverage or a delivery date.

## Registry and site development

```sh
python3 check.py
cargo agents plugin validate . --no-check-crates
cargo run --locked --manifest-path site/Cargo.toml -- dist --base /symposiast-registry/
python3 site/check_links.py dist --base /symposiast-registry/
```

The [site development guide](site/README.md) describes local preview, catalog fields, and GitHub Pages deployment. The site reads the catalog and exported provenance directly; keep contribution instructions in `CONTRIBUTING.md`, which also supplies the website's contribution page.

## Source notices

Stucco source excerpts retain their upstream MIT or Apache-2.0 licenses. Both license texts are included in the skill's evidence: [MIT](stucco-0-2-1/skills/stucco-0a431c79-guide/references/evidence/e0029.txt) and [Apache-2.0](stucco-0-2-1/skills/stucco-0a431c79-guide/references/evidence/e0030.txt). The source URLs and hashes identify the retained excerpts. Preserve these notices when redistributing the guidance.
