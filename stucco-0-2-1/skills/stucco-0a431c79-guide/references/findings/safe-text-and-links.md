# Keep untrusted content in escaped text and checked URLs

Pass user-provided text to text-rendering components or element .text(...) builders so Stucco escapes markup. Link URLs pass through Href, which accepts relative URLs and the http, https, mailto and tel schemes and replaces disallowed schemes such as javascript: with #. Reserve Raw::trusted for markup that the application already trusts; it bypasses normal escaping. Do not set reserved component attributes to override behavior.

Basis: Documented.

Features: typography.

## Evidence

- [e0022 lines 47–58](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/components.md))
