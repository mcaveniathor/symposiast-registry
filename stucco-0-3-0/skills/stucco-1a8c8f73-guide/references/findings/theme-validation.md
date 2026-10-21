# Validate themes and choose scheme ownership

Use presets or seeded themes for reproducible appearance within this release. Handle custom-theme contrast failures with Theme::build or Bundle::try_new instead of relying on Bundle::new's panic. Seed mappings may change in minor releases. Use server_theme for a server-owned scheme preference; it omits the localStorage initializer but does not disable component scripts. Custom CSS should use semantic tokens in stucco.utilities.

Basis: Documented.

Features: No additional features stated.

## Evidence

- [e0004 lines 103–121](../evidence/e0004.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/README.md))
- [e0005 lines 303–306](../evidence/e0005.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/CHANGELOG.md))
- [e0030 lines 333–350](../evidence/e0030.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/recipes.md))
- [e0031 lines 211–219](../evidence/e0031.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/theming.md))
