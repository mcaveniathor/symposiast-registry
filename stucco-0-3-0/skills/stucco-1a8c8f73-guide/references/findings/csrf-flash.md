# Configure CSRF and flash layers explicitly

Install CsrfLayer and FlashLayer explicitly before the final stucco call; enabling axum alone does not install them. Configure the browser-visible origin and a stable secret key. Include the issued token on unsafe requests. Keep secure cookies in production and align CSRF buffering with request body limits. Flash cookies are signed, readable, short-lived feedback, not authorization or secret storage.

Basis: Documented.

Features: axum, forms, feedback.

## Evidence

- [e0026 lines 101–159](../evidence/e0026.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/forms.md))
- [e0026 lines 185–198](../evidence/e0026.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/forms.md))
