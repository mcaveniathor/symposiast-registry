# Registry website

The public catalog is built with Stucco 0.2.1 and deployed to GitHub Pages at `https://mcaveniathor.github.io/symposiast-registry/`. It is a static site: searching happens in the browser and requests/submissions go through GitHub. Browsing and installation instructions work without JavaScript; search and copy controls are progressive enhancements.

The header's dark mode switch uses Stucco's `data-theme` attribute and `stucco-theme` storage key. It follows the system preference until a visitor chooses a mode, restores that choice before first paint, and shares it between both sites when hosted on the same origin. With JavaScript disabled, the switch stays hidden and the page follows the system preference. Custom site colors use `light-dark()` alongside Stucco's theme.

From the registry root:

```sh
python3 check.py
cargo run --locked --manifest-path site/Cargo.toml -- dist --base /symposiast-registry/
python3 site/check_links.py dist --base /symposiast-registry/
```

For a local root-path preview:

```sh
cargo run --locked --manifest-path site/Cargo.toml -- dist --base /
python3 -m http.server 8000 --directory dist
```

Open `http://localhost:8000`. The generator validates `--base` and supports root or project hosting. Use a fresh output directory when removing entries so stale pages are not retained; CI always starts from a fresh checkout.

## Content and catalog

`catalog.json` lists each entry directory, a concise `summary`, searchable `tags`, and an optional `bundle_release` tag. Use `null` until a real release download exists. Every portable entry must have exactly one catalog record. `check.py` verifies this coverage and the recorded evidence/lockfile hashes.

The site derives crate name, exact release, recipes, compiler versions, and review status from each skill's `references/provenance.json`. `CONTRIBUTING.md` is rendered directly as the contribution page. Other guides live in `site/content/`; the shell and catalog renderer are in `site/src/`; CSS and browser behavior are in `site/assets/`. The Markdown renderer, link checker, and base visual styles originate in the Symposiast documentation site.

The generator site's [maintainer guide](https://mcaveniathor.github.io/symposiast/maintainers.html) owns the pilot and publishing workflow. Keep this site's installation, feedback, and contribution links aligned with it. Consumer reports use `usage-feedback.yml` in this repository; maintainer pilots use `maintainer-pilot.yml` in the generator repository. Keep exported skills focused on task guidance.

For Rust changes, run `cargo fmt --manifest-path site/Cargo.toml --check` and `cargo clippy --locked --manifest-path site/Cargo.toml --all-targets -- -D warnings`, plus the site build and link checker. Check both desktop and narrow layouts, search by crate/version/task, the empty state, and copy/install links. External links are not fetched by the local checker.

## Deployment

The Registry website workflow validates the catalog, builds pages, checks local links and assets, and uploads a Pages artifact. Pull requests build without deploying. Pushes to `main` deploy after the build succeeds.

Enable GitHub Pages with **GitHub Actions** as the source in the repository's **Settings → Pages**. The `github-pages` environment must permit deployments from `main`. No separate hosting account or credentials are needed. A workflow dispatch can rebuild the current default branch.
