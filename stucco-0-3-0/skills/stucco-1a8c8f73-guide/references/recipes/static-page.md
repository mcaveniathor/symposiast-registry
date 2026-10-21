# Export a static page with its assets

Write a page and its bundle assets into public. Serve public as the web root so /_stucco/ URLs resolve.

Release: stucco 0.3.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = false
features = [
    "layout",
    "typography",
]
version = "=0.3.0"
```

```rust
use std::fs;
use stucco::prelude::*;

fn main() -> std::io::Result<()> {
    let bundle = Bundle::new(Preset::Slate);
    fs::create_dir_all("public/_stucco")?;
    for path in bundle.paths() {
        let file = bundle.get(&path).expect("listed asset exists");
        fs::write(format!("public{path}"), &*file.bytes)?;
    }
    let html = Page::new(&bundle, "Hello")
        .main(Container::new().child(
            Stack::new()
                .child(Heading::new(1, "Hello"))
                .child(Text::new("Rendered in Rust.")),
        ))
        .render();
    fs::write("public/index.html", html)?;
    Ok(())
}

```

## Evidence

- [e0003 lines 8–18](../evidence/e0003.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/README.md))
- [e0030 lines 179–195](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/recipes.md))
- [e0001 lines 87–94](../evidence/e0001.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/Cargo.toml))
