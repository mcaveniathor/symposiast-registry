# Creating a schedule does not schedule its execution

init_schedule creates a missing schedule and edit_schedule can configure it. The Main schedule only runs labels listed in MainScheduleOrder. For caller-driven ticks, run the custom schedule explicitly; for a phase inside App::update, insert its label into MainScheduleOrder. App::update is one update call, not a wall-clock fixed-timestep guarantee.

Basis: Inferred from the cited sources.

Features: std.

## Evidence

- [e0005 lines 137–164](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
- [e0005 lines 1254–1288](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
- [e0009 lines 211–246](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
- [e0009 lines 288–305](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
