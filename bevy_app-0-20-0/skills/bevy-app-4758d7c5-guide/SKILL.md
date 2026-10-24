---
name: "bevy-app-4758d7c5-guide"
description: "Use bevy_app 0.20.0 for the tasks covered in its release guidance; consult cited findings, recipe verification status, and required features."
depends-on: "bevy_app==0.20.0"
---

# bevy_app 0.20.0

Focused bevy_app 0.20.0 guidance for reusable synchronous plugins and caller-driven simulation schedules, with default features disabled and std enabled. The recipes use bevy_ecs =0.20.0 with the same feature selection. Browser rendering, web bindings, networking, asynchronous plugin readiness, fixed wall-clock pacing, and deterministic replay are outside this coverage. Analysis is imported from frozen published-crate evidence; no separate agent review is recorded.

Use this guidance only with the stated release. Check the project's Cargo.lock before applying it. A compiled recipe passed cargo check; runtime behavior has not been tested. Read each recipe's feature and dependency requirements.

## Guidance

- [Configure reusable behavior through Plugin::build](references/findings/plugin-composition.md) — Documented.
- [Creating a schedule does not schedule its execution](references/findings/schedule-initialization-and-driving.md) — Inferred from the cited sources.
- [Use the 0.20.0 executor API](references/findings/single-threaded-executor.md) — Inferred from the cited sources.
- [Keep the simulation feature set explicit](references/findings/minimal-std-features.md) — Inferred from the cited sources.

## Recipes

- [Drive a plugin-owned simulation schedule explicitly](references/recipes/external-simulation-tick.md) — Compiled.
- [Insert a custom phase into App::update](references/recipes/main-loop-custom-phase.md) — Compiled.

## Documentation examples

13 examples inventoried. [Adaptations, supporting examples, and skipped examples with reasons](references/examples.md); [complete inventory and coverage data](references/examples.json). This accounts only for the collected evidence.

## Coverage limitations

- Collection is bounded to 48 UTF-8 evidence files, 48 KiB per file, and 320 KiB total; examples and API documentation are sampled, not exhaustive. Rendered documentation and external documentation sites are not crawled.
- Pinned repository snapshot unavailable; using published crate evidence: download exceeds size limit: https://codeload.github.com/bevyengine/bevy/tar.gz/b3bdd75becaffe9a16677cf5bae5bd6d198f052b
- Evidence is a bounded prefix of https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs (49152 bytes retained).

## Provenance

[Release, evidence hashes, review status, and compilation records](references/provenance.json). Paths in that record are relative to its directory. Dependency lockfiles are included alongside the records.
