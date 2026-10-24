# Reproduce bevy_app 0.20.0 guidance

This entry was collected, imported, verified, and exported with Symposiast revision [`b7cdadb4b128621c4f088d5e6ad74d4304888c88`](https://github.com/mcaveniathor/symposiast/tree/b7cdadb4b128621c4f088d5e6ad74d4304888c88). The analysis is an explicit import derived from frozen published-crate evidence. No nested provider invocation, separate agent review, or human approval is claimed.

With that generator built, use a new bundle directory and the checked-in analysis:

```sh
symposiast collect bevy_app --version 0.20.0 --output /tmp/bevy_app-0.20.0
symposiast analyze /tmp/bevy_app-0.20.0 --analysis ./analysis.json
symposiast verify /tmp/bevy_app-0.20.0 --offline
symposiast export-registry /tmp/bevy_app-0.20.0 --output /tmp/bevy_app-0-20-0
```

`analyze` resolves and checks recipes online by default. In the original run the network-restricted check could not reach crates.io; the retained analysis stage was successfully recovered with `verify --offline` using cached dependencies. No source or recipe correction was needed. The export contains current successful native compilation records and recorded dependency lockfiles. Fresh resolution can select newer transitive versions; compare against the exported lockfiles before claiming identical inputs.

The published archive identifies Bevy commit `b3bdd75becaffe9a16677cf5bae5bd6d198f052b`. The repository tarball exceeded collection limits; all guidance citations refer to published-crate evidence. API files are bounded and the documentation inventory reports omissions. This is focused coverage of the recipes, not a complete Bevy manual.

The upstream MIT and Apache-2.0 notices were retrieved separately at that exact commit because the bounded collection did not contain them. They are retained in `../notices/` with their URLs and SHA256 hashes; they were not inserted into the generated bundle's source provenance.

Verification uses `rustc 1.99.1 (58cf8f9c3 2026-10-13)` on Linux `x86_64-unknown-linux-gnu`, with default features disabled and only `std` requested for both Bevy component crates. All three recipe programs across the two entries were separately run with `cargo run --locked --offline` and their assertions passed. Each recipe also passed `cargo check --locked --offline --all-targets --target wasm32-unknown-unknown`. These are additional checks; the exported verification schema records native cargo-check results only. WebAssembly programs were compiled but not executed in a browser. Browser runtime behavior, rendering, transport, load capacity, and deterministic replay are outside this entry.
