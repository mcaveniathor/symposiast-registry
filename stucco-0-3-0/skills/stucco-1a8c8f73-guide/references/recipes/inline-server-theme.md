# Render an inline document with a server-selected scheme

Render a standalone document with an explicit dark scheme and inline assets. This example uses basic elements and no component behaviors; inline output cannot receive fragments.

Release: stucco 0.3.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = false
features = []
version = "=0.3.0"
```

```rust
use stucco::{el, Bundle, Delivery, Page};
use stucco::theme::{Preset, Scheme};

fn main() {
    let bundle = Bundle::new(Preset::Slate);
    let html = Page::new(&bundle, "Account")
        .server_theme(Some(Scheme::Dark))
        .delivery(Delivery::Inline)
        .body(el::main().child(el::h1().text("Account")))
        .render();
    println!("{html}");
}

```

## Evidence

- [e0006 lines 19–24](../evidence/e0006.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/src/lib.rs))
- [e0024 lines 48–50](../evidence/e0024.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/enhancement.md))
- [e0030 lines 333–350](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/recipes.md))
