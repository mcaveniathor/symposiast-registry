# Serve full pages and a named fragment

Serve an ordinary GET form and negotiate the greeting fragment when the two Stucco request headers select it. Client-side fragment fetching and swapping are not included.

Release: stucco 0.3.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.axum]
default-features = true
features = []
version = "0.8"

[dependencies.stucco]
default-features = true
features = ["axum"]
version = "=0.3.0"

[dependencies.tokio]
default-features = true
features = [
    "macros",
    "rt-multi-thread",
    "net",
]
version = "1"
```

```rust
use std::collections::HashMap;
use axum::{extract::Query, response::Response, routing::get, Router};
use stucco::prelude::*;

fn greeting(name: &str) -> impl Render {
    el::div().id("greeting")
        .child(Text::new(format!("Hello, {name}!")))
}

async fn index(page: PageCx, Query(params): Query<HashMap<String, String>>) -> Response {
    let name = params.get("q").map(String::as_str).unwrap_or("visitor");
    page.respond()
        .fragment("greeting", || greeting(name))
        .page(|| {
            page.title("Greeting").main(Stack::of((
                Heading::new(1, "Greeting"),
                Form::get("/").child(Stack::of((
                    Field::new("Name", Input::text("q").value(name)),
                    Button::new("Greet").submit(),
                ))),
                greeting(name),
            )))
        })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/", get(index)).stucco(Preset::Slate);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}

```

## Evidence

- [e0004 lines 65–66](../evidence/e0004.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/README.md))
- [e0007 lines 6–63](../evidence/e0007.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/examples/hello/src/main.rs))
- [e0024 lines 9–26](../evidence/e0024.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/enhancement.md))
