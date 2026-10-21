# Match table controls to backend capabilities

Use collections for typed tables; add derive when columns should come from a struct. Reuse column_specs for query parsing. Controls require both column declarations and backend capabilities; marking a column does not implement querying. Cursor totals may be unknown, filtering can scan many records, and the example's numbered mode scans all matches. Scope queries to the requester independently of cursor validation.

Basis: Documented.

Features: collections, derive.

## Evidence

- [e0020 lines 37–78](../evidence/e0020.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/collections.md))
- [e0020 lines 88–110](../evidence/e0020.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/collections.md))
- [e0020 lines 164–171](../evidence/e0020.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/collections.md))
- [e0010 lines 19–23](../evidence/e0010.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/examples/orders/README.md))
