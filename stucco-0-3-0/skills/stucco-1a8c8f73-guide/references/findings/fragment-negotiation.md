# Keep fragment responses paired with a full-page fallback

Build ordinary navigation and forms first. PageCx::respond selects named fragments using Stucco-Request and Stucco-Target headers and falls back to the full page for other targets. Browser request and swapping logic is still needed. Use Page::enhanced when a page must receive fragments before any component behavior has loaded the runtime.

Basis: Documented.

Features: axum.

## Evidence

- [e0024 lines 7–26](../evidence/e0024.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/enhancement.md))
- [e0024 lines 30–46](../evidence/e0024.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/enhancement.md))
