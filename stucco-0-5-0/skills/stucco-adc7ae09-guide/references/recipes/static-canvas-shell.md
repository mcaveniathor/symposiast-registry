# Export a static canvas shell with content-hashed assets

Generate index.html, export all Stucco bundle assets, and assert a unique canvas ID, labelled player input, linked stylesheet, and absence of Stucco script tags. Accepts an optional output directory. Does not load a game, execute WebAssembly, or verify a browser rendering.

Release: stucco 0.5.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = false
features = [
    "layout",
    "typography",
    "forms",
]
version = "=0.5.0"
```

```rust
use std::{env, fs, io, path::PathBuf};
use stucco::prelude::*;
use stucco::{Raw, theme::Scheme};

fn main() -> io::Result<()> {
    let output = env::args_os().nth(1).map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("public"));
    let bundle = Bundle::new(Preset::Slate);
    // Only application-authored static markup belongs in Raw::trusted.
    let canvas = Raw::trusted(r#"<canvas id="game-canvas" width="960" height="540" aria-label="Game view">Your browser needs canvas support.</canvas>"#);
    let html = Page::new(&bundle, "Game shell")
        .server_theme(Some(Scheme::Dark))
        .main(Stack::new()
            .child(Heading::new(1, "Game shell"))
            .child(Field::new("Player name", Input::text("name").id("player-name")))
            .child(canvas))
        .render();

    assert_eq!(html.matches("id=\"game-canvas\"").count(), 1);
    assert!(html.contains("<canvas "));
    assert!(html.contains("for=\"player-name\""));
    assert!(!html.contains("<script"));
    assert!(html.contains(bundle.stylesheet_url()));

    fs::create_dir_all(&output)?;
    for path in bundle.paths() {
        let file = bundle.get(&path).expect("listed bundle path exists");
        let destination = output.join(path.trim_start_matches('/'));
        fs::create_dir_all(destination.parent().expect("asset parent"))?;
        fs::write(destination, file.bytes.as_ref())?;
    }
    fs::write(output.join("index.html"), html)?;
    assert!(output.join(bundle.stylesheet_url().trim_start_matches('/')).is_file());
    Ok(())
}

```

## Evidence

- [e0036 lines 179–195](../evidence/e0036.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/recipes.md))
- [e0036 lines 333–353](../evidence/e0036.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/recipes.md))
- [e0028 lines 47–58](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/components.md))
- [e0011 lines 20–38](../evidence/e0011.txt) ([source](https://docs.rs/crate/stucco/0.5.0/source/Cargo.toml.orig))
- [e0032 lines 8–28](../evidence/e0032.txt) ([source](https://github.com/mcaveniathor/stucco/blob/0bc5845c04bc4c841ca855edde0d86247985dd63/docs/guide/forms.md))
