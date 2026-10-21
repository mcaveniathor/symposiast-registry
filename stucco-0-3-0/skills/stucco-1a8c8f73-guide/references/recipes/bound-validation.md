# Render validation errors from returned form state

Demonstrate validation and bound error markup without an HTTP server. A deployed POST flow must supply its handler, CSRF setup and issued token, and a 422 response on validation failure.

Release: stucco 0.3.0.

Verification: Compiled: cargo check --locked --all-targets passed; runtime behavior was not tested (rustc 1.99.1 (58cf8f9c3 2026-10-13)).
Compilation does not establish runtime behavior or idiomatic usage.

```toml
[dependencies.stucco]
default-features = false
features = ["forms"]
version = "=0.3.0"
```

```rust
use stucco::prelude::*;
use stucco::{to_html, FormState, Validator};

fn validate(state: FormState) -> Result<String, FormState> {
    let mut validator = Validator::new(state);
    let customer = validator.text("customer")
        .required("Enter a customer name")
        .max_chars(80, "Use 80 characters or fewer")
        .get();
    validator.finish(|| customer)
}

fn main() {
    let submitted = FormState::new().with_value("customer", "");
    match validate(submitted) {
        Ok(customer) => println!("Validated customer: {customer}"),
        Err(state) => {
            let form = Form::post("/customers")
                .child(ErrorSummary::new(&state).field("customer", "customer-name"))
                .child(
                    Field::new("Customer", Input::text("customer").id("customer-name"))
                        .required()
                        .bind(&state),
                )
                .child(Button::new("Save").submit());
            println!("{}", to_html(&form));
        }
    }
}

```

## Evidence

- [e0026 lines 44–99](../evidence/e0026.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/forms.md))
- [e0012 lines 97–107](../evidence/e0012.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/examples/orders/src/form.rs))
- [e0030 lines 83–101](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/recipes.md))
- [e0001 lines 82–85](../evidence/e0001.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/Cargo.toml))
