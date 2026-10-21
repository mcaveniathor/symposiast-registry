# Keep conflict recovery and return navigation explicit

Compare revisions and save atomically in storage; with_conflict only records UI state. Return 409 while retaining submitted values and the submitted revision. If ErrorSummary and ConflictNotice are both shown, disable focus on the notice. Revalidate ReturnContext on each request with application-owned allowlists; it prevents external return redirects but does not authorize access or validate filter values.

Basis: Documented.

Features: forms.

## Evidence

- [e0026 lines 200–233](../evidence/e0026.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/forms.md))
- [e0020 lines 221–232](../evidence/e0020.txt) ([source](https://github.com/mcaveniathor/stucco/blob/07335e73dfb22aba2ce89487bcee634a24c2080e/docs/guide/collections.md))
