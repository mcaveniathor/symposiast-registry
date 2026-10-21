# Use Page::main for the main landmark and skip link

Compose the document with Page::new(&bundle, title).main(content).render(). PageExt::main supplies the <main id="main"> landmark and a preceding skip link; avoid adding a second main landmark inside it. With default-features = false, enable layout for this shortcut and layout components, and typography for Heading, Text and Link. The examples here target the published 0.2.1 facade and its recorded repository commit, so newer repository HEAD examples can require unavailable APIs.

Basis: Documented.

Features: layout, typography.

## Evidence

- [e0006 lines 60–82](../evidence/e0006.txt) ([source](https://docs.rs/crate/stucco/0.2.1/source/src/lib.rs))
- [e0025 lines 36–56](../evidence/e0025.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/getting-started.md))
- [e0022 lines 5–19](../evidence/e0022.txt) ([source](https://github.com/mcaveniathor/stucco/blob/303ee2433ce3c05d0af9c4b79a4e09958d374f92/docs/guide/components.md))
- [e0002 lines 1–6](../evidence/e0002.txt) ([source](https://docs.rs/crate/stucco/0.2.1/source/.cargo_vcs_info.json))
