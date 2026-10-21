# Separate stored values from table presentation

For money-like values, follow the orders example's separation: retain integer cents in the model, parse decimal input with checked integer arithmetic, and format cells independently. Its floating-point dollars projection supports table filtering; it is not evidence of exact decimal arithmetic for general financial calculations.

Basis: Inferred from the cited sources.

Features: collections.

## Evidence

- [e0021 lines 13–29](../evidence/e0021.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/examples/orders/src/model.rs))
- [e0012 lines 63–79](../evidence/e0012.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/examples/orders/src/form.rs))
- [e0020 lines 88–88](../evidence/e0020.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/collections.md))
