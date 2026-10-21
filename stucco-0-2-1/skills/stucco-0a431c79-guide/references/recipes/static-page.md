# Export a static page with hashed assets

Write index.html and every Bundle asset to an optional output directory (default stucco-example). BASE_PATH is /demo/ for a project site; change it to / for root hosting. Assertions check escaping, the main landmark, the skip link and exact asset bytes on disk.

Release: stucco 0.2.1.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = false
features = [
    "layout",
    "typography",
]
version = "=0.2.1"
```

```rust
use std::{
    fs,
    path::{Component, Path, PathBuf},
};
use stucco::prelude::*;

// Match the site's URL mount point; use "/" for a site served at the root.
const BASE_PATH: &str = "/demo/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("stucco-example"));
    let bundle = Bundle::new(Preset::Slate).prefix(&format!("{BASE_PATH}_stucco/"));
    let user_text = "A <careful> welcome & a fresh start.";
    let html = Page::new(&bundle, "Built with Stucco")
        .main(
            Container::new().child(
                Stack::new()
                    .child(Heading::new(1, "Built with Stucco"))
                    .child(Text::new(user_text))
                    .child(Link::new(
                        "Stucco repository",
                        "https://github.com/mcaveniathor/stucco",
                    )),
            ),
        )
        .render();

    assert!(html.contains(r#"<main id="main">"#));
    assert!(html.contains(r##"href="#main""##));
    assert!(html.contains("A &lt;careful&gt; welcome &amp; a fresh start."));
    assert!(!html.contains("<careful>"));
    assert!(html.contains(bundle.stylesheet_url()));

    fs::create_dir_all(&output)?;
    let paths = bundle.paths();
    for url in &paths {
        // The deployment prefix is a URL, not a directory inside the output.
        let relative = url
            .strip_prefix(BASE_PATH)
            .expect("asset stays under site prefix");
        let relative = Path::new(relative);
        assert!(
            relative
                .components()
                .all(|part| matches!(part, Component::Normal(_)))
        );
        let destination = output.join(relative);
        fs::create_dir_all(destination.parent().expect("asset has a parent"))?;
        let asset = bundle.get(url).expect("listed asset exists");
        fs::write(&destination, &*asset.bytes)?;
        assert_eq!(fs::read(&destination)?, asset.bytes.as_ref());
    }
    fs::write(output.join("index.html"), &html)?;
    assert_eq!(fs::read_to_string(output.join("index.html"))?, html);
    assert!(!paths.is_empty());
    println!(
        "Wrote {} and {} assets; serve it at {BASE_PATH}",
        output.display(),
        paths.len()
    );
    Ok(())
}

```

## Evidence

- [e0025 lines 36–56](../evidence/e0025.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/getting-started.md))
- [e0022 lines 49–58](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/components.md))
- [e0026 lines 144–160](../evidence/e0026.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/recipes.md))
- [e0028 lines 103–124](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
- [e0028 lines 152–168](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
