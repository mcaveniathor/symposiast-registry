# Derive columns and format stored cents

Render table markup from derived columns with separate numeric and display values. This has no backend capabilities or query execution; embed the table in a Page for themed document output.

Release: stucco 0.3.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = false
features = ["derive"]
version = "=0.3.0"
```

```rust
use stucco::prelude::*;
use stucco::{to_html, CollectionPage};

#[derive(Columns)]
struct Order {
    #[col(label = "ID", sortable)]
    id: u64,
    #[col(sortable, searchable)]
    customer: String,
    #[col(key = "total", label = "Total", value = dollars, display = money)]
    total_cents: u64,
}

fn dollars(order: &Order) -> f64 {
    order.total_cents as f64 / 100.0
}

fn money(order: &Order) -> String {
    format!("${}.{:02}", order.total_cents / 100, order.total_cents % 100)
}

fn main() {
    let page = CollectionPage {
        rows: vec![Order {
            id: 7,
            customer: "<Ada>".into(),
            total_cents: 4800,
        }],
        next: None,
        prev: None,
        total: None,
    };
    let table = DataTable::from_page(&page, "Orders").columns(Order::columns());
    let html = to_html(&table);
    assert!(html.contains("&lt;Ada&gt;"));
    assert!(html.contains("$48.00"));
    println!("{html}");
}

```

## Evidence

- [e0001 lines 77–80](../evidence/e0001.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/Cargo.toml))
- [e0009 lines 4–34](../evidence/e0009.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/tests/derive.rs))
- [e0009 lines 50–74](../evidence/e0009.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/tests/derive.rs))
- [e0020 lines 90–106](../evidence/e0020.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/collections.md))
