# Reproduce the Stucco 0.5.0 static shell guidance

This entry reuses frozen evidence from an existing exact-release bundle and imports a focused analysis. It does not claim a live provider invocation, separate agent review, human review, or upstream endorsement. Both the analysis and all retained evidence are included. Sixty collected documentation examples are accounted for; two are adapted, one is supporting, and the others are explicitly outside this entry's scope.

The generator is [Symposiast b7cdadb4b128621c4f088d5e6ad74d4304888c88](https://github.com/mcaveniathor/symposiast/tree/b7cdadb4b128621c4f088d5e6ad74d4304888c88), from its clean source checkout. An older installed 0.1.0 binary rejected the additive example-inventory field; the current pinned build was then used successfully. No plugin was installed, enabled, or synchronized for this work.

## Source identity

- Published package: `stucco = 0.5.0` from crates.io.
- Archive SHA256: `19f39b7ebfa02bb654fd49f10d5a2064da3a0e89c11e0e90a7b22f936f083a8c`.
- Packaged repository commit: `0bc5845c04bc4c841ca855edde0d86247985dd63`.
- Recipe features: `default-features = false`, `features = ["layout", "typography", "forms"]`.
- The live crates.io lookup and locally cached published archive checksum matched this release. The verification lockfile resolves the published package and records the same checksum.
- Upstream license notices are retained separately under `notices/`, with their exact commit URLs and content hashes. The sampled font license files are retained among the frozen evidence.

## Reproduction

Use the pinned generator revision above with Rust 1.99.1. From the registry root, restore the saved evidence into a new directory:

```sh
python3 stucco-0-5-0/reproduce/restore.py /tmp/stucco-0.5.0-bundle
symposiast analyze /tmp/stucco-0.5.0-bundle --analysis /tmp/stucco-0.5.0-bundle/analysis.json --timeout 300
symposiast verify /tmp/stucco-0.5.0-bundle --offline --timeout 300
cargo run --locked --offline --manifest-path /tmp/stucco-0.5.0-bundle/verification/static-canvas-shell/Cargo.toml -- /tmp/stucco-static-shell-output
symposiast export-registry /tmp/stucco-0.5.0-bundle --output /tmp/stucco-0-5-0-export
```

Analysis verification requires registry network access if dependencies are not cached. The restore script verifies every frozen evidence hash before reconstructing the unanalysed bundle. It does not copy or invent compilation records; analysis and verification produce fresh records. The analysis is also reusable after `symposiast collect stucco --version 0.5.0`, provided that the resulting source IDs, evidence hashes, and documentation inventory match the included evidence.

## Observed checks and limits

On Linux x86_64 with `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, the recipe passed `cargo check --locked --all-targets` through Symposiast. Separately, `cargo run --locked --offline` passed the recipe's assertions and exported `index.html` and the bundle files. Assertions cover a unique literal canvas ID, an input label wired to its explicit ID, a linked stylesheet that was written to disk, and absence of script tags in this simple shell. These runtime assertions are separate from the generated compilation record.

The sample does not supply a game loader, execute WebAssembly, run a browser, render a game, or test multiplayer behavior. Stable canvas lifetime during subsequent UI updates remains the application's responsibility. `server_theme` omits the theme initializer but does not suppress scripts needed by enhanced navigation or behavior-bearing components. Exporting all bundle files can include scripts that the page never references.

Collection limits remain visible in the generated skill and provenance: selection was bounded to 48 UTF-8 files, 48 KiB per file and 320 KiB total; five eligible files were omitted, and one repository special-file entry was rejected. This is focused static-export guidance, not comprehensive coverage of Stucco 0.5.0. No immutable full bundle release has been published; `bundle_release` remains `null`.
