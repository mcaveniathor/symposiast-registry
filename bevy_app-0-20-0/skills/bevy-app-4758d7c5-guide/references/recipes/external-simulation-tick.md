# Drive a plugin-owned simulation schedule explicitly

A synchronous plugin installs a custom schedule with the 0.20.0 SingleThreadedExecutor API. An external caller advances exactly three simulation ticks. It supplies no wall-clock pacing, networking, renderer, or browser event loop.

Release: bevy_app 0.20.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.101.0-nightly (0abfedbc7 2026-10-02)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.bevy_app]
default-features = false
features = ["std"]
version = "=0.20.0"

[dependencies.bevy_ecs]
default-features = false
features = ["std"]
version = "=0.20.0"
```

```rust
use bevy_app::{App, Plugin, PluginsState};
use bevy_ecs::{prelude::*, schedule::{ScheduleLabel, SingleThreadedExecutor}};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct SimulationTick;

#[derive(Resource, Default)]
struct Tick(u64);

struct SimulationPlugin;
impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tick>()
            .init_schedule(SimulationTick)
            .edit_schedule(SimulationTick, |schedule| {
                schedule.set_executor(SingleThreadedExecutor::new());
            })
            .add_systems(SimulationTick, advance_tick);
    }
}

fn advance_tick(mut tick: ResMut<Tick>) {
    tick.0 += 1;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(SimulationPlugin);
    // These plugins are synchronous; an async plugin needs readiness polling.
    assert_eq!(app.plugins_state(), PluginsState::Ready);
    app.finish();
    app.cleanup();
    for _ in 0..3 {
        app.world_mut().run_schedule(SimulationTick);
    }
    assert_eq!(app.world().resource::<Tick>().0, 3);
}

```

## Evidence

- [e0011 lines 5–59](../evidence/e0011.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/plugin.rs))
- [e0005 lines 229–295](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
- [e0005 lines 1254–1288](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
- [e0009 lines 288–327](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
