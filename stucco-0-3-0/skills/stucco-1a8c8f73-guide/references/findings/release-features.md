# Select features from the 0.3.0 manifest

Use Rust 1.85 or later. Default features include overlay; axum, collections, derive, and icons require explicit selection. derive enables collections and its component dependencies. For migration from 0.2, remove the reserved marketing/diagram/markdown/askama/maud features and use into_http for plain HTTP response conversion.

Basis: Documented.

Features: No additional features stated.

## Evidence

- [e0001 lines 12–16](../evidence/e0001.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/Cargo.toml))
- [e0001 lines 50–94](../evidence/e0001.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/Cargo.toml))
- [e0005 lines 9–17](../evidence/e0005.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/CHANGELOG.md))
