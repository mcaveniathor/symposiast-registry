# Preserve rendering and application responsibility boundaries

Use normal rendering for escaped text and checked URLs; reserve Raw::trusted for trusted markup. Use component builders instead of overriding reserved attributes. For dialogs that must work without script, provide link_opener with a real fallback page. Displayed readiness checks do not grant permission: recheck eligibility and authorization when executing actions.

Basis: Documented.

Features: No additional features stated.

## Evidence

- [e0022 lines 49–58](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/components.md))
- [e0022 lines 224–227](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/components.md))
- [e0022 lines 263–265](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/components.md))
