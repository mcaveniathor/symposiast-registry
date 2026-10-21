# Install registry guidance

The registry distributes portable skills through [Symposium](https://symposium.dev/). Using an entry requires Symposium and a configured agent, but does not require Symposiast or an analysis-agent login.

Check the [catalog](index.html) for your exact crates.io release first. The current entry is Stucco 0.2.1. This community registry is separate from Symposium's official recommendations; you subscribe explicitly below. If your version is missing, [request coverage](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml) instead of changing your project's dependencies to match the catalog.

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

## 4. Try a real task

Reload your agent session if needed. Ask it to consult the installed release skill and identify the relevant recipe before adapting it to your project. For a project already using Stucco 0.2.1:

> Use the installed Stucco 0.2.1 guidance to add a static page with a content-hashed stylesheet. Check the recipe's feature requirements and explain how the exported assets match the deployment path.

Run your project's build and behavior checks on the result. The [static-page recipe](https://github.com/mcaveniathor/symposiast-registry/blob/main/stucco-0-2-1/skills/stucco-0a431c79-guide/references/recipes/static-page.md) is a starting point; its recorded compilation does not validate your modified application.

Installation, consultation, and task success are separate observations. Check the agent's visible explanation or tool activity for use of a finding or recipe; if you cannot tell, report that uncertainty. Try another task or session when you next need the crate and note whether the guidance is still useful.

[Share first or repeat use](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=usage-feedback.yml), including failed attempts. A short redacted description of the task, setup, and outcome is enough. Reports are voluntary and public; do not include credentials, private source, or full conversations.

## If guidance does not appear

- Check `Cargo.lock` for the exact version and source; a similarly named path or Git dependency is not established as equivalent to the analyzed release.
- Confirm the registry block is in the configuration used by your `SYMPOSIUM_HOME` and your agent is configured. Run `cargo agents sync` and inspect its diagnostics, then `cargo agents status` in the consumer workspace.
- Inspect the installed skill and reload the session. A successful sync does not establish that the agent consulted it.
- Request a missing release, or [report an installation problem](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=correction.yml) with redacted diagnostics and tool versions. For the correction form's item field, say "installation" when no recipe is involved.

## Need the full bundle?

Use the release page's full-bundle download when available. Download both the archive and its `.sha256` file, then verify the archive before extracting it. For the pilot:

```sh
sha256sum -c stucco-0.2.1-r1.sha256
```

The [Stucco bundle release](https://github.com/mcaveniathor/symposiast-registry/releases/tag/stucco-0.2.1-r1) includes runnable recipe projects, logs, frozen sources, and lockfiles. To enable MCP retrieval and the version hook, install Symposiast, extract the bundle, and run `symposiast install` with the extracted bundle directory, then sync in the consumer workspace. See [full-bundle installation](https://mcaveniathor.github.io/symposiast/getting-started.html).

Use one distribution of a release's skill in a workspace: the portable entry and full runtime plugin intentionally share its identity.

## Missing a release?

[Request guidance](https://github.com/mcaveniathor/symposiast-registry/issues/new?template=guidance-request.yml) with the crate, version, and tasks you need. You can also [submit an entry](contributing.html) yourself.
