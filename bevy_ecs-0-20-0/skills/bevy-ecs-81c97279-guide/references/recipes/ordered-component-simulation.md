# Run resource-configured movement and observe the result

A complete standalone World and Schedule apply two fixed integer simulation steps and sum moving-entity positions after each movement system. Integer values keep this small example exact; this is not a general determinism guarantee.

Release: bevy_ecs 0.20.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.101.0-nightly (0abfedbc7 2026-10-02)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.bevy_ecs]
default-features = false
features = ["std"]
version = "=0.20.0"
```

```rust
use bevy_ecs::prelude::*;

#[derive(Component, Debug, PartialEq, Eq)]
struct Position(i64);
#[derive(Component)]
struct Velocity(i64);
#[derive(Resource)]
struct Step(i64);
#[derive(Resource, Default)]
struct Summary { ticks: u64, position_sum: i64 }

fn movement(step: Res<Step>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * step.0;
    }
}
fn summarize(query: Query<&Position, With<Velocity>>, mut summary: ResMut<Summary>) {
    summary.ticks += 1;
    summary.position_sum = query.iter().map(|position| position.0).sum();
}
fn main() {
    let mut world = World::new();
    world.insert_resource(Step(2));
    world.insert_resource(Summary::default());
    let moving = world.spawn((Position(0), Velocity(3))).id();
    let stationary = world.spawn(Position(10)).id();
    let mut schedule = Schedule::default();
    schedule.add_systems((movement, summarize).chain());
    schedule.run(&mut world);
    schedule.run(&mut world);
    assert_eq!(world.entity(moving).get::<Position>(), Some(&Position(12)));
    assert_eq!(world.entity(stationary).get::<Position>(), Some(&Position(10)));
    let summary = world.resource::<Summary>();
    assert_eq!((summary.ticks, summary.position_sum), (2, 12));
}

```

## Evidence

- [e0003 lines 27–110](../evidence/e0003.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/README.md))
- [e0003 lines 123–180](../evidence/e0003.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/README.md))
- [e0010 lines 14–48](../evidence/e0010.txt) ([source](https://docs.rs/crate/bevy_ecs/0.20.0/source/examples/resources.rs))
