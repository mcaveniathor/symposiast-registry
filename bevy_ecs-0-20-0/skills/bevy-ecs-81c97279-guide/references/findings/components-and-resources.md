# Separate per-entity state from world-wide configuration

Represent per-entity state as Component types and singleton simulation configuration or counters as Resource types. Systems express their data access through Query, Res, and ResMut. The complete recipe combines the published movement and resource examples.

Basis: Documented.

Features: std.

## Evidence

- [e0003 lines 27–110](../evidence/e0003.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/README.md))
- [e0003 lines 123–158](../evidence/e0003.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/README.md))
