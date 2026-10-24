# Use the 0.20.0 executor API

The published MainSchedulePlugin configures simple facilitator schedules with schedule.set_executor(SingleThreadedExecutor::new()). The recipes apply the same release-specific API to custom schedules. Single-threaded execution alone does not establish deterministic query order or deterministic cross-platform simulation.

Basis: Inferred from the cited sources.

Features: std.

## Evidence

- [e0009 lines 1–11](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
- [e0009 lines 308–327](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
