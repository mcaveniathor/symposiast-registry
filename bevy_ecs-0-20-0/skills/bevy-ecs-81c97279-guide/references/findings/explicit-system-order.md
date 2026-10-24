# Order dependent simulation phases explicitly

Systems with semantic dependencies need explicit schedule ordering. The published resource example chains a writer and reader; the change-detection example uses system sets and after. Use chain for a small linear sequence, and named sets or explicit before/after constraints for broader plugin composition. Ordering systems does not make entity iteration or floating-point behavior deterministic.

Basis: Inferred from the cited sources.

Features: std.

## Evidence

- [e0003 lines 112–117](../evidence/e0003.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/README.md))
- [e0010 lines 14–30](../evidence/e0010.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/examples/resources.rs))
- [e0005 lines 26–42](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/examples/change_detection.rs))
- [e0005 lines 58–63](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/examples/change_detection.rs))
