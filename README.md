# Symposiast registry

Release-specific Rust crate guidance for [Symposium](https://symposium.dev/), generated and validated with [Symposiast](https://github.com/mcaveniathor/symposiast).

Browse the [registry website](https://mcaveniathor.github.io/symposiast-registry/), [submit an entry](CONTRIBUTING.md#submit-verified-guidance), or [request a crate/release](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml).

This is an independent community registry, separate from [Symposium's official recommendations](https://github.com/symposium-dev/recommendations). Entries here require an explicit subscription and do not imply upstream acceptance or endorsement. Symposiast generates and maintains the guidance; Symposium distributes it.

**Maintaining a crate?** [Start a pilot](https://mcaveniathor.github.io/symposiast/maintainers.html) for one release and a few real user tasks. **Using guidance?** [Try it on a task](https://mcaveniathor.github.io/symposiast-registry/subscribe.html#4-try-a-real-task) and [share first or repeat use](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=usage-feedback.yml).

## Catalog

| Crate | Release | Recipes | Verification | Guidance |
| --- | --- | --- | --- | --- |
| Stucco | 0.3.0 | Static pages, Axum fragments, status components, derived tables, validation and themes | Six recipes compiled; agent review approved | [Read the skill](stucco-0-3-0/skills/stucco-1a8c8f73-guide/SKILL.md) |

The registry entry includes verified recipes, dependency lockfiles, and frozen sources. [Download the full bundle and SHA256 checksum](https://github.com/mcaveniathor/symposiast-registry/releases/tag/stucco-0.3.0-r1).

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

The Stucco skill matches `stucco==0.3.0`. Each release's skill has a distinct identity. Unsupported releases do not receive another release's guidance. This registry supplies skills and their references; it does not register hooks or MCP servers or require the Symposiast executable.

The portable entry is validated with Symposium 0.5.1. Confirm the resolved version and source in your own workspace before applying a recipe. Avoid installing both this entry and the full runtime plugin for the same release in one workspace, since they share the skill identity.

After syncing, reload your agent session if needed and ask it to consult the installed release skill for a concrete task. Check that it actually used a finding or recipe, adapt the example to your features, and run your project's checks. If no entry matches, request your release rather than changing dependencies just to activate guidance. The [installation guide](https://mcaveniathor.github.io/symposiast-registry/subscribe.html) includes a Stucco task and troubleshooting steps.

## Evidence and verification

The skill includes all frozen evidence and a [provenance record](stucco-0-3-0/skills/stucco-1a8c8f73-guide/references/provenance.json), including source URLs and hashes, original release metadata, compilation fingerprints, and dependency lockfiles. Paths in the record are relative to its directory. Compilation establishes that the recorded recipe inputs passed Cargo checks; it does not establish runtime behavior. The original bundle schema does not record the verification platform.

The Stucco analysis was generated with Codex, refined through a structured analysis import, reviewed with Codex, and compiled with Rust 1.99.1 on Linux x86_64. All six recipes have current successful compilation records. No consumer runtime trials or human review are claimed. The active 0.2.1 entry was removed when 0.3.0 was published; historical download archives remain available.

## Submit guidance, request coverage, or report a correction

Read [CONTRIBUTING.md](CONTRIBUTING.md) for the complete generation, verification, export, pull-request, and maintainer review process.

- **Have guidance to share?** Fork the registry, export a verified bundle into a new entry directory, update `catalog.json`, run the checks, and submit a pull request.
- **Need a crate or release?** Search [existing issues](https://github.com/mcaveniathor/symposiast-registry/issues), then [request guidance](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml). Include the package, version, tasks, and feature/platform requirements. No bundle is needed to make a request.
- **Found incorrect guidance?** [Report a correction](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=correction.yml) with the entry, recipe/finding, and reproduction.
- **Tried it in a project?** [Report first or repeat use](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=usage-feedback.yml), including failed setup attempts. Installation, agent consultation, and task success are distinct observations; a short public-safe description is enough.

Different crate releases stay in separate directories. Corrections get a new registry commit and bundle revision; published archives are not replaced. Requests do not guarantee coverage or a delivery date.

## Help improve guidance

Help improve Symposiast and the registry by reviewing recipes, reporting corrections, maintaining coverage, or sharing a useful release with other developers. [Join a maintainer pilot](https://mcaveniathor.github.io/symposiast/maintainers.html) to help make guidance useful across repeated tasks and releases.

## Registry and site development

```sh
python3 check.py
cargo agents plugin validate . --no-check-crates
cargo run --locked --manifest-path site/Cargo.toml -- dist --base /symposiast-registry/
python3 site/check_links.py dist --base /symposiast-registry/
```

The [site development guide](site/README.md) describes local preview, catalog fields, and GitHub Pages deployment. The site reads the catalog and exported provenance directly; keep contribution instructions in `CONTRIBUTING.md`, which also supplies the website's contribution page.

## Source notices

Stucco source excerpts retain their upstream MIT or Apache-2.0 licenses. Both license texts are retained alongside the exported guidance: [MIT](stucco-0-3-0/notices/LICENSE-MIT) and [Apache-2.0](stucco-0-3-0/notices/LICENSE-APACHE), with [source hashes](stucco-0-3-0/notices/provenance.json). The source URLs and hashes identify the retained excerpts. Preserve these notices when redistributing the guidance.
