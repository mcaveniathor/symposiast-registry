# Serve a document and its stylesheet through Axum

Return a Document from a PageCx handler, install Stucco last on the router, and issue in-memory requests for the HTML and its content-hashed CSS. Assertions cover status, content type, escaped text, landmarks, immutable cache policy and exact stylesheet bytes.

Release: stucco 0.2.1.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.axum]
default-features = false
features = []
version = "0.8.9"

[dependencies.stucco]
default-features = false
features = [
    "layout",
    "typography",
    "axum",
]
version = "=0.2.1"

[dependencies.tokio]
default-features = true
features = [
    "macros",
    "rt",
]
version = "1.53"

[dependencies.tower]
default-features = true
features = ["util"]
version = "0.5.3"
```

```rust
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{
        Request, StatusCode,
        header::{CACHE_CONTROL, CONTENT_TYPE},
    },
    routing::get,
};
use stucco::prelude::*;
use tower::ServiceExt;

async fn index(page: PageCx) -> Document {
    page.title("Hello, Stucco").main(
        Container::new().child(
            Stack::new()
                .child(Heading::new(1, "Hello, Stucco"))
                .child(Text::new("Rust <HTML> & a real stylesheet.")),
        ),
    )
}

fn request(uri: &str) -> Request<Body> {
    Request::builder().uri(uri).body(Body::empty()).unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bundle = Bundle::new(Preset::Slate);
    let stylesheet_url = bundle.stylesheet_url().to_owned();
    let stylesheet = bundle
        .get(&stylesheet_url)
        .expect("theme stylesheet exists");
    // Call stucco last so it provides assets, middleware and PageCx to every route.
    let app = Router::new().route("/", get(index)).stucco(bundle);

    let response = app.clone().oneshot(request("/")).await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[CONTENT_TYPE], "text/html; charset=utf-8");
    let html = String::from_utf8(to_bytes(response.into_body(), 1024 * 1024).await?.to_vec())?;
    assert!(html.contains(r#"<main id="main">"#));
    assert!(html.contains(r##"href="#main""##));
    assert!(html.contains("Rust &lt;HTML&gt; &amp; a real stylesheet."));
    assert!(html.contains(&stylesheet_url));

    let response = app.oneshot(request(&stylesheet_url)).await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[CONTENT_TYPE], "text/css; charset=utf-8");
    assert!(
        response.headers()[CACHE_CONTROL]
            .to_str()?
            .contains("immutable")
    );
    let css = to_bytes(response.into_body(), 1024 * 1024).await?;
    assert_eq!(css.as_ref(), stylesheet.bytes.as_ref());
    println!("Verified HTML and {stylesheet_url} through in-memory requests.");
    Ok(())
}

```

## Evidence

- [e0025 lines 58–82](../evidence/e0025.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/getting-started.md))
- [e0006 lines 110–111](../evidence/e0006.txt) ([source](https://docs.rs/crate/stucco/0.2.1/source/src/lib.rs))
- [e0021 lines 1–25](../evidence/e0021.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/examples/orders/tests/http.rs))
- [e0028 lines 121–124](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
- [e0028 lines 163–168](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
- [e0014 lines 20–28](../evidence/e0014.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/Cargo.toml))
