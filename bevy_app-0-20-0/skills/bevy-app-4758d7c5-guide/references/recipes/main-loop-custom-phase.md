# Insert a custom phase into App::update

A function plugin places a custom schedule immediately after Update so every caller-driven App::update executes it once. The counter assertion distinguishes initialization from actual execution; it is not a fixed-rate clock.

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
use bevy_app::{App, MainScheduleOrder, PluginsState, Update};
use bevy_ecs::{prelude::*, schedule::{ScheduleLabel, SingleThreadedExecutor}};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct PublishState;

#[derive(Resource, Default)]
struct Counters { updates: u64, published: u64 }

fn update_state(mut counters: ResMut<Counters>) {
    counters.updates += 1;
}
fn publish_state(mut counters: ResMut<Counters>) {
    counters.published = counters.updates;
}
fn publishing_plugin(app: &mut App) {
    app.init_resource::<Counters>()
        .init_schedule(PublishState)
        .edit_schedule(PublishState, |schedule| {
            schedule.set_executor(SingleThreadedExecutor::new());
        })
        .add_systems(Update, update_state)
        .add_systems(PublishState, publish_state);
    app.world_mut().resource_mut::<MainScheduleOrder>()
        .insert_after(Update, PublishState);
}
fn main() {
    let mut app = App::new();
    app.add_plugins(publishing_plugin);
    assert_eq!(app.plugins_state(), PluginsState::Ready);
    app.finish();
    app.cleanup();
    app.update();
    app.update();
    let counters = app.world().resource::<Counters>();
    assert_eq!((counters.updates, counters.published), (2, 2));
}

```

## Evidence

- [e0011 lines 24–37](../evidence/e0011.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/plugin.rs))
- [e0005 lines 137–164](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
- [e0005 lines 229–295](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
- [e0009 lines 211–246](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
- [e0009 lines 288–327](../evidence/e0009.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/main_schedule.rs))
