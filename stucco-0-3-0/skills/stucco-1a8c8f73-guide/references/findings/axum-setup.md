# Finish router construction with stucco

Add axum 0.8 and Tokio with macros, rt-multi-thread, and net. Call Router::stucco last, after routes and state; handlers take PageCx and return Document. Content renders when assigned, allowing request-local borrows. Use stucco_with and LayerConfig to change the documented 1 MiB body limit and 30-second timeout.

Basis: Documented.

Features: axum.

## Evidence

- [e0004 lines 65–85](../evidence/e0004.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/README.md))
- [e0028 lines 78–82](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/getting-started.md))
- [e0030 lines 197–208](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/recipes.md))
