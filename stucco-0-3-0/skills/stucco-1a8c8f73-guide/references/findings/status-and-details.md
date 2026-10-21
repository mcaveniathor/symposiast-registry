# Compose native status, navigation, progress and disclosures

Use Badge labels to communicate status independently of tone; mark Avatar decorative only when the same name is adjacent. DescriptionList accepts rich Render values, and Stat changes remain neutral until change_tone is set. Breadcrumbs marks the current page, while Steps uses a zero-based current index. Progress and Meter carry visible labels. Disclosure and Accordion use native details/summary; exclusive accordions depend on browser support. Callout is static content rather than a live announcement. These components live in existing default feature families.

Basis: Documented.

Features: data, navigation, feedback, layout.

## Evidence

- [e0022 lines 106–202](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/components.md))
- [e0001 lines 50–94](../evidence/e0001.txt) ([source](https://docs.rs/crate/stucco/0.3.0/source/Cargo.toml))
