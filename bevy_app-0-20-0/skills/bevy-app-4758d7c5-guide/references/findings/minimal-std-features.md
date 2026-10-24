# Keep the simulation feature set explicit

The bevy_app default feature set includes reflection, bevy_ecs defaults, and the error panic hook. These recipes disable defaults on both component crates and enable std. The separately declared web feature adds platform web support and browser dependencies; the minimal simulation recipes do not provide a browser event loop or browser API integration. Cargo feature unification can enable additional features in a larger consumer graph.

Basis: Inferred from the cited sources.

Features: std.

## Evidence

- [e0001 lines 38–85](../evidence/e0001.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/Cargo.toml))
- [e0001 lines 94–109](../evidence/e0001.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/Cargo.toml))
