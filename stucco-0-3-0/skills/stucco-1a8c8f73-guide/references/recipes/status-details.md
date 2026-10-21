# Render status and details with native components

Render a standalone order page demonstrating all eleven new display, navigation, progress and disclosure components. Replace sample values and navigation URLs with application data and routes.

Release: stucco 0.3.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = true
features = []
version = "=0.3.0"
```

```rust
use stucco::prelude::*;
use stucco::Delivery;

fn main() {
    let bundle = Bundle::new(Preset::Slate);
    let content = Stack::new()
        .child(PageHeader::new("Order 1042").breadcrumbs(
            Breadcrumbs::new().link("Orders", "/orders").current("Order 1042"),
        ))
        .child(Cluster::of((
            Avatar::new("Ada Lovelace").decorative(),
            Text::new("Ada Lovelace"),
        )))
        .child(DescriptionList::new()
            .item("Status", Badge::new("Paid").tone(Tone::Success))
            .item("Total", "$128.00"))
        .child(Stat::new("Orders", "128")
            .change("12%", Trend::Up)
            .change_tone(Tone::Success)
            .context("vs last week"))
        .child(Steps::new("Checkout")
            .step("Cart", "/checkout/cart")
            .step("Delivery", "/checkout/delivery")
            .step_disabled("Payment")
            .current(1))
        .child(Progress::new("Import", 42.0, 100.0).show_value())
        .child(Meter::new("Storage", 7.2, 0.0, 10.0)
            .low(3.0).high(8.0).optimum(1.0).value_text("7.2 of 10 GB"))
        .child(Callout::new(Tone::Info)
            .title("Preparing for dispatch")
            .child(Text::new("Your order will ship within two working days.")))
        .child(Disclosure::new("Shipping details")
            .heading(2).child(Text::new("Tracked delivery is included.")))
        .child(Accordion::new().heading(2)
            .item_open("Shipping", Text::new("Arrives by Friday."))
            .item("Returns", Text::new("Return within 30 days."))
            .exclusive());
    let html = Page::new(&bundle, "Order 1042")
        .delivery(Delivery::Inline).main(content).render();
    println!("{html}");
}

```

## Evidence

- [e0022 lines 106–202](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/components.md))
- [e0030 lines 36–69](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/recipes.md))
- [e0024 lines 48–50](../evidence/e0024.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/enhancement.md))
