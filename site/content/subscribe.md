# Install registry guidance

The registry distributes portable skills through [Symposium](https://symposium.dev/). Using an entry requires Symposium and a configured agent, but does not require Symposiast or an analysis-agent login.

## 1. Add the registry

Install and configure Symposium using its [getting started guide](https://symposium.dev/). Add this block to your existing `~/.symposium/config.toml`, or the configuration under your `SYMPOSIUM_HOME`. Keep your existing agent and registry settings.

```toml
[[registry]]
name = "symposiast"
git = "https://github.com/mcaveniathor/symposiast-registry"
```

## 2. Sync your Rust workspace

From the consuming project:

```sh
cargo agents sync
cargo agents status
```

Each skill matches an exact crate release. Check the resolved version and source in your project's `Cargo.lock`, then inspect the installed skills. A path dependency, Git revision, or different published version is not established as equivalent to the analyzed crates.io package. Registry updates follow your Symposium configuration.

The Stucco pilot targets `stucco==0.2.1`. It was checked with Symposium 0.5.1 in isolated Claude Code and Codex configurations, including repeated sync and removal after a version mismatch. This check does not establish compatibility with every agent or Symposium version.

## 3. Inspect before applying

Open a release in the [catalog](index.html), read the skill, and check recipe features, citations, collection warnings, compilation records, and review status. A successful Cargo check does not establish runtime behavior. An absent recorded agent review does not mean approval.

Portable entries include skills, frozen evidence, provenance, and recorded lockfiles. They do not install executables, hooks, or MCP servers.

## Need the full bundle?

Use the release page's full-bundle download when available. Download both the archive and its `.sha256` file, then verify the archive before extracting it. For the pilot:

```sh
sha256sum -c stucco-0.2.1-r1.sha256
```

The [Stucco bundle release](https://github.com/mcaveniathor/symposiast-registry/releases/tag/stucco-0.2.1-r1) includes runnable recipe projects, logs, frozen sources, and lockfiles. To enable MCP retrieval and the version hook, install Symposiast, extract the bundle, and run `symposiast install` with the extracted bundle directory, then sync in the consumer workspace. See [full-bundle installation](https://mcaveniathor.github.io/symposiast/getting-started.html).

Use one distribution of a release's skill in a workspace: the portable entry and full runtime plugin intentionally share its identity.

## Missing a release?

[Request guidance](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml) with the crate, version, and tasks you need. You can also [submit an entry](contributing.html) yourself.
