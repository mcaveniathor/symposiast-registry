# Write hashed assets and match the deployment URL prefix

A Bundle supplies content-hashed stylesheets and scripts. For static output, write every Bundle::paths URL using the bytes returned by Bundle::get; rendering index.html alone leaves its stylesheet unavailable. Bundle::prefix changes the asset URL prefix and normalizes leading and trailing slashes. For a GitHub Pages project mounted at /demo/, set the asset prefix to /demo/_stucco/ and strip /demo/ when mapping those URLs into the artifact directory. Keep the output paths relative and retain the hash-bearing filenames. At the domain root, use /_stucco/ instead. This deployment mapping follows the documented prefix and static-export APIs; the recipe asserts that all emitted assets reach disk.

Basis: Documented.

Features: No additional features stated.

## Evidence

- [e0025 lines 36–56](../evidence/e0025.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/getting-started.md))
- [e0026 lines 144–160](../evidence/e0026.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/recipes.md))
- [e0028 lines 103–124](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
- [e0028 lines 152–168](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
- [e0028 lines 208–222](../evidence/e0028.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/crates/stucco-core/src/bundle.rs))
