# Configure reusable behavior through Plugin::build

Plugin::build configures the App when the plugin is registered. Both fn(&mut App) and a struct implementing Plugin work; plugins are unique by default. Put resource and system registration in build, and expose game configuration through the plugin type. The recipe uses synchronous setup; callers embedding App must respect readiness, finish, and cleanup for plugins that need them.

Basis: Documented.

Features: std.

## Evidence

- [e0011 lines 5–99](../evidence/e0011.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/plugin.rs))
- [e0005 lines 229–295](../evidence/e0005.txt) ([source](https://docs.rs/crate/bevy_app/0.20.0/source/src/app.rs))
