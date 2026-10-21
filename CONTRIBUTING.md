# Contribute to the registry

You can submit release-specific guidance, request coverage, or report a correction. You do not need to be a crate maintainer to contribute. This is community guidance; inclusion does not imply endorsement by the crate's authors.

## Request a crate or release

Search the [existing requests](https://github.com/mcaveniathor/symposiast-registry/issues) first, then [open a guidance request](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml). Include:

- The crates.io package name and exact release (or say that you need help choosing a release).
- The upstream repository and the tasks you want recipes for.
- Required features, target platform, and any related crates or constraints.
- Links to documentation or examples, and whether you can help test the result.

Requests do not require a generated bundle, credentials, or code. Maintainers can clarify scope and contributors can volunteer in the issue. A request is not a promise of coverage or a delivery date. Please keep API keys and private source code out of issues.

## Submit verified guidance

1. Fork this registry and create a branch. Search open issues and pull requests for overlapping work; link a related request in your PR if one exists.
2. Install Symposiast from a revision that includes `export-registry`, and install/configure [Symposium](https://symposium.dev/). The export command is currently on the generator's `feat/portable-registry-export` branch; this known revision includes it:

   ```sh
   cargo install --git https://github.com/mcaveniathor/symposiast \
     --rev b34ec10e8603bc64c2bc8abce6bbe466e404f119 --locked symposiast
   symposiast export-registry --help
   ```

   [Inspect that generator revision](https://github.com/mcaveniathor/symposiast/tree/b34ec10e8603bc64c2bc8abce6bbe466e404f119). Use the repository's pinned toolchain if your installed Rust is too old. Once the export command is available in your chosen newer revision, you can use that revision instead and record it in your PR.

   Generate a bundle for an **exact published crates.io release**, using your chosen authenticated agent. For example, replace the package, version, and paths below:

   ```sh
   symposiast build your-crate --version 1.2.3 --agent codex \
     --output /tmp/your-crate-1.2.3
   ```

   Claude Code is also supported with `--agent claude-code`. An optional `--reviewer` records a separate agent review. See [the generator guide](https://mcaveniathor.github.io/symposiast/getting-started.html) for collection, analysis imports, repair, and refresh. Do not claim an agent review for imported analysis unless one is actually recorded.
3. Inspect the findings, citations, collection warnings, feature requirements, and recipe code. Preserve upstream license notices with the frozen evidence. Every recipe needs a current successful compilation record and its recorded lockfile. If the inputs changed, run:

   ```sh
   symposiast verify /tmp/your-crate-1.2.3
   ```

   Cargo checks can execute dependency build scripts and procedural macros; use a disposable environment for untrusted packages. Record any runtime checks separately: compilation alone does not establish runtime behavior.
4. Export into a **new, nonexistent** directory at the registry root:

   ```sh
   symposiast export-registry /tmp/your-crate-1.2.3 \
     --output ./your-crate-1-2-3
   ```

   Keep both generated `Symposium.toml` and `SYMPOSIUM.toml`, the skill, provenance, evidence, and lockfiles together. Do not add local executable paths, MCP registrations, or hooks to portable entries. Use a separate entry for each crate release. For a correction, export outside the existing entry, inspect the diff, then replace the entry's generated files; do not hand-edit provenance to make validation pass.
5. Add an object to `catalog.json` with the entry directory, a concise task summary, searchable tags, and `bundle_release` (a published release tag, or `null` until maintainers publish it). The site derives crate/version, recipe counts, and review status from the exported provenance. Never invent a download link. Keep the README catalog current.
6. Run the registry and site checks from the registry root:

   ```sh
   python3 check.py
   cargo agents plugin validate . --no-check-crates
   cargo run --locked --manifest-path site/Cargo.toml -- dist --base /symposiast-registry/
   python3 site/check_links.py dist --base /symposiast-registry/
   ```

   `check.py` checks recorded evidence and lockfile hashes; plugin validation checks the manifest structure. These checks do not repeat recipe compilation or establish recommendation quality. `--no-check-crates` skips live crate lookups; maintainers should confirm the published source identity when reviewing.
7. Open a pull request using the checklist. Describe scope and limitations, link the exact generator revision, include the verification toolchain and platform, and provide a downloadable full source bundle or reproducible generation details. Keep secrets, local agent state, and build caches out of commits. Do not commit the full source bundle into the portable entry.

## Review and publication

Maintainers review source identity, citations, license notices, feature requirements, recipe usefulness, verification records, and the generated-file diff. An absent agent review is visible and is not approval. Unresolved recorded review issues and stale or failed compilation block export. CI must pass before inclusion; contributors may be asked to revise or narrow the scope. Human review is still needed after automated checks pass.

After acceptance, maintainers merge the entry and publish any full bundle under a new revision tag such as `your-crate-1.2.3-r1`, with a SHA256 checksum file. The archive should retain `bundle.json`, frozen sources, runnable verification projects, logs, and lockfiles. Set `bundle_release` only after the release exists. Existing release downloads are immutable: corrections use `r2`, `r3`, and so on. The registry entry follows the latest accepted correction for that exact crate release, while old archives remain available.

The GitHub Pages catalog rebuilds from the repository on each push to `main`. No separate site submission is needed. Consumers pick up registry changes according to their Symposium update configuration, then run `cargo agents sync` in the consumer workspace.

## Report a correction

[Open a correction report](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=correction.yml) with the entry path, crate/version, recipe or finding ID, current registry commit if known, expected behavior, and a minimal reproduction. Include the dependency features and target where relevant. If you already have a fix, submit a PR with regenerated evidence and verification records as needed. For a new crate version, use a guidance request and a separate entry rather than rewriting the older release.
