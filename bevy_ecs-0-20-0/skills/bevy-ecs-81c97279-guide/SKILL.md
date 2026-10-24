---
name: "bevy-ecs-81c97279-guide"
description: "Use bevy_ecs 0.20.0 for the tasks covered in its release guidance; consult cited findings, recipe verification status, and required features."
depends-on: "bevy_ecs==0.20.0"
---

# bevy_ecs 0.20.0

Focused bevy_ecs 0.20.0 guidance for standalone component simulation with resources and explicitly ordered systems. The recipe disables default features and enables std, and has no renderer, network stack, or browser APIs. Analysis is imported from frozen published-crate evidence; no separate agent review is recorded.

Use this guidance only with the stated release. Check the project's Cargo.lock before applying it. A compiled recipe passed cargo check; runtime behavior has not been tested. Read each recipe's feature and dependency requirements.

## Guidance

- [Separate per-entity state from world-wide configuration](references/findings/components-and-resources.md) — Documented.
- [Order dependent simulation phases explicitly](references/findings/explicit-system-order.md) — Inferred from the cited sources.
- [Match the feature selection before reusing a recipe](references/findings/std-only-feature-selection.md) — Inferred from the cited sources.

## Recipes

- [Run resource-configured movement and observe the result](references/recipes/ordered-component-simulation.md) — Compiled.

## Documentation examples

41 examples inventoried. [Adaptations, supporting examples, and skipped examples with reasons](references/examples.md); [complete inventory and coverage data](references/examples.json). This accounts only for the collected evidence.

## Coverage limitations

- Collection is bounded to 48 UTF-8 evidence files, 48 KiB per file, and 320 KiB total; examples and API documentation are sampled, not exhaustive. Rendered documentation and external documentation sites are not crawled.
- Pinned repository snapshot unavailable; using published crate evidence: download exceeds size limit: https://codeload.github.com/bevyengine/bevy/tar.gz/b3bdd75becaffe9a16677cf5bae5bd6d198f052b
- Evidence is a bounded prefix of https://docs.rs/crate/bevy_ecs/0.20.0/source/src/lib.rs (49152 bytes retained).
- Evidence is a bounded prefix of https://docs.rs/crate/bevy_ecs/0.20.0/source/src/bundle/info.rs (23045 bytes retained).
- 137 distinct eligible source files were omitted by the evidence budget; 23 files selected.

## Provenance

[Release, evidence hashes, review status, and compilation records](references/provenance.json). Paths in that record are relative to its directory. Dependency lockfiles are included alongside the records.
