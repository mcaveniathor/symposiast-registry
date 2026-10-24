# Match the feature selection before reusing a recipe

The default features include std, reflection, async_executor, and backtrace; multi_threaded is a separate feature. This recipe uses default-features=false and features=[std] and intentionally does not request multi_threaded. That is a minimal simulation configuration, not a guarantee about all target dependencies or a browser runtime test. Check the complete consumer feature graph and compile its intended target.

Basis: Inferred from the cited sources.

Features: std.

## Evidence

- [e0001 lines 47–104](../evidence/e0001.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/Cargo.toml))
