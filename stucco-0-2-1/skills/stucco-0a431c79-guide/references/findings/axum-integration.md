# Enable axum explicitly and apply Stucco after the routes

Enable the optional axum feature to expose PageCx, Document and StuccoRouter through stucco::prelude. A handler can extract PageCx, call title(...).main(...), and return Document; the content is rendered when set and may borrow request data. Call Router::stucco last, after routes and with_state, to install asset serving, theme extraction and the standard middleware across every route. The static-page recipe needs no Axum dependency. The Axum recipe uses Tower in-memory requests to exercise both HTML and the actual theme stylesheet without opening a listener.

Basis: Documented.

Features: axum, layout, typography.

## Evidence

- [e0006 lines 41–45](../evidence/e0006.txt) ([source](https://docs.rs/crate/stucco/0.2.1/source/src/lib.rs))
- [e0006 lines 110–111](../evidence/e0006.txt) ([source](https://docs.rs/crate/stucco/0.2.1/source/src/lib.rs))
- [e0025 lines 58–82](../evidence/e0025.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/getting-started.md))
- [e0021 lines 1–25](../evidence/e0021.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/examples/orders/tests/http.rs))
- [e0014 lines 20–28](../evidence/e0014.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/Cargo.toml))
