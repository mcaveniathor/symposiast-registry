mod markdown;
mod shell;

use serde::Deserialize;
use shell::Site;
use std::{fs, io, path::Path};
use stucco::{Asset, el, register_asset};

const REPO: &str = "https://github.com/mcaveniathor/symposiast-registry";
const DOCUMENTS: [(&str, &str, &str); 3] = [
    (
        "subscribe",
        "Install guidance",
        "Subscribe with Symposium and sync guidance for your exact crate release.",
    ),
    (
        "contributing",
        "Contribute",
        "Submit verified guidance, request coverage, or report a correction.",
    ),
    (
        "about",
        "About the registry",
        "Understand evidence, compilation records, review status, and the registry's scope.",
    ),
];
static SITE_STYLE: Asset = Asset {
    name: "symposiast-registry-site",
    css: Some(include_str!("../assets/site.css")),
    behavior: None,
    deps: &[],
};
register_asset!(SITE_STYLE);

#[derive(Deserialize)]
struct Entry {
    entry: String,
    summary: String,
    tags: Vec<String>,
    bundle_release: Option<String>,
}
#[derive(Deserialize)]
struct Release {
    name: String,
    version: String,
    repository: Option<String>,
}
#[derive(Deserialize)]
struct Recipe {
    recipe_id: String,
    status: String,
    rustc_version: String,
}
#[derive(Deserialize)]
struct Review {
    approved: bool,
    issues: Vec<serde_json::Value>,
}
#[derive(Deserialize)]
struct Provenance {
    release: Release,
    recipes: Vec<Recipe>,
    review: Option<Review>,
}
struct Record {
    entry: Entry,
    provenance: Provenance,
    skill: String,
}

fn review_label(record: &Record) -> &'static str {
    match &record.provenance.review {
        None => "No recorded agent review",
        Some(review) if review.approved && review.issues.is_empty() => "Agent review approved",
        Some(_) => "Review has unresolved issues",
    }
}
fn normalize_base(base: &str) -> io::Result<String> {
    if !base.starts_with('/')
        || base.contains("//")
        || !base
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"/-_.".contains(&c))
        || base.split('/').any(|p| p == "." || p == "..")
    {
        return Err(io::Error::other(
            "base must be an absolute URL path without dot segments",
        ));
    }
    Ok(format!("{}/", base.trim_end_matches('/')))
}
fn load(root: &Path) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let entries: Vec<Entry> =
        serde_json::from_str(&fs::read_to_string(root.join("catalog.json"))?)?;
    let mut records = Vec::new();
    for entry in entries {
        if entry.entry.is_empty()
            || !entry
                .entry
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
        {
            return Err("invalid catalog entry directory".into());
        }
        let skills =
            fs::read_dir(root.join(&entry.entry).join("skills"))?.collect::<Result<Vec<_>, _>>()?;
        let mut found = Vec::new();
        for skill in skills {
            let path = skill.path().join("references/provenance.json");
            if path.is_file() {
                found.push((skill.file_name().to_string_lossy().into_owned(), path));
            }
        }
        if found.len() != 1 {
            return Err(format!("{} must contain exactly one release skill", entry.entry).into());
        }
        let (skill, path) = found.pop().unwrap();
        let provenance = serde_json::from_str(&fs::read_to_string(path)?)?;
        records.push(Record {
            entry,
            provenance,
            skill,
        });
    }
    records.sort_by(|a, b| {
        (&a.provenance.release.name, &a.provenance.release.version)
            .cmp(&(&b.provenance.release.name, &b.provenance.release.version))
    });
    Ok(records)
}
fn catalog(site: &Site, records: &[Record]) -> String {
    let mut list = el::div().id("entries").class("catalog-list");
    for record in records {
        let p = &record.provenance;
        list = list.child(
            el::article()
                .class("catalog-entry")
                .attr(
                    "data-search",
                    format!(
                        "{} {} {} {}",
                        p.release.name,
                        p.release.version,
                        record.entry.summary,
                        record.entry.tags.join(" ")
                    )
                    .to_lowercase(),
                )
                .child(
                    el::div()
                        .class("entry-name")
                        .child(el::p().class("eyebrow").text("RUST / CRATES.IO"))
                        .child(
                            el::h3().child(
                                el::a()
                                    .href(site.url(&format!("{}.html", record.entry.entry)))
                                    .text(&p.release.name),
                            ),
                        )
                        .child(el::span().class("version").text(&p.release.version)),
                )
                .child(
                    el::div()
                        .class("entry-description")
                        .child(el::p().text(&record.entry.summary))
                        .child(el::p().class("tags").text(record.entry.tags.join(" / "))),
                )
                .child(
                    el::div()
                        .class("entry-status")
                        .child(el::p().class("verified").text(format!(
                            "{} / {} recipes compiled",
                            p.recipes.iter().filter(|r| r.status == "compiled").count(),
                            p.recipes.len()
                        )))
                        .child(el::p().class("review-status").text(review_label(record)))
                        .child(
                            el::a()
                                .class("text-link")
                                .href(site.url(&format!("{}.html", record.entry.entry)))
                                .text("Explore release →"),
                        ),
                ),
        );
    }
    site.page("Registry", "Release-specific Rust crate guidance, with frozen evidence and compilation records.", "home",
        el::div()
            .child(el::section().class("registry-hero")
                .child(el::div().child(el::p().class("eyebrow").text("THE SYMPOSIAST REGISTRY"))
                    .child(el::h1().text("Crate knowledge.").child(el::br()).child(el::em().text("Version attached.")))
                    .child(el::p().class("hero-description").text("Practical Rust guidance for the release you actually use. Browse recipes, inspect their evidence, and bring the right skills into your workspace."))
                    .child(el::div().class("hero-links")
                        .child(el::a().class("primary-link").href(site.url("subscribe.html")).text("Install guidance ↗"))
                        .child(el::a().class("text-link").href(format!("{REPO}/issues/new?template=guidance-request.yml")).text("Request a crate →"))))
                .child(el::aside().class("registry-note")
                    .child(el::p().class("eyebrow").text("INSIDE EVERY ENTRY"))
                    .child(el::ol().child(el::li().text("An exact published crate release"))
                        .child(el::li().text("Recipes with compilation records"))
                        .child(el::li().text("Frozen sources and provenance")))
                    .child(el::p().text("Portable skills for Symposium. No Symposiast CLI or analysis-agent login needed."))
                    .child(el::p().text("An independent community source. ")
                        .child(el::a().href(site.url("about.html#part-of-the-symposium-ecosystem")).text("How it fits with Symposium →")))))
            .child(el::section().class("catalog-section").aria("label", "Release catalog")
                .child(el::div().class("section-heading").child(el::h2().text("Browse the registry"))
                    .child(el::p().class("eyebrow").text(format!("{} RELEASE{}", records.len(), if records.len() == 1 { "" } else { "S" }))))
                .child(el::div().id("search-controls").attr("hidden", "")
                    .child(el::label().attr("for", "search").text("Search crates, versions, or tasks"))
                    .child(el::input().id("search").attr("type", "search").attr("placeholder", "Try stucco, 0.2.1, or static sites").attr("autocomplete", "off"))
                    .child(el::p().id("result-count").attr("role", "status").attr("aria-live", "polite")))
                .child(list)
                .child(el::div().id("empty-state").attr("hidden", "")
                    .child(el::h3().text("No matching releases yet."))
                    .child(el::p().text("Try another crate, version, or task — or tell us what you need."))
                    .child(el::a().href(format!("{REPO}/issues/new?template=guidance-request.yml")).text("Request guidance →")))
                .child(el::p().class("catalog-footnote").text("Compilation checks the recorded recipe inputs. It does not establish runtime behavior or crate-maintainer endorsement.")))
            .child(el::section().class("contribute-strip")
                .child(el::div().child(el::p().class("eyebrow").text("GROW THE COLLECTION"))
                    .child(el::h2().text("Make guidance useful in real projects."))
                    .child(el::p().text("Maintain a crate? Start with one release and three user tasks. Tried a recipe? Tell us what worked and whether you used it again.")))
                .child(el::div().class("hero-links")
                    .child(el::a().class("primary-link").href("https://mcaveniathor.github.io/symposiast/maintainers.html").text("Start a maintainer pilot →"))
                    .child(el::a().class("text-link").href(format!("{REPO}/issues/new?template=usage-feedback.yml")).text("Share your experience →"))
                    .child(el::a().class("text-link").href(site.url("contributing.html")).text("Submit guidance →")))))
}
fn detail(site: &Site, record: &Record) -> String {
    let p = &record.provenance;
    let source = format!(
        "{REPO}/blob/main/{}/skills/{}",
        record.entry.entry, record.skill
    );
    let mut recipes = el::ul().class("recipe-list");
    for recipe in &p.recipes {
        recipes = recipes.child(
            el::li()
                .child(
                    el::a()
                        .href(format!(
                            "{source}/references/recipes/{}.md",
                            recipe.recipe_id
                        ))
                        .text(&recipe.recipe_id),
                )
                .child(el::span().text(format!(" — {}", recipe.status)))
                .child(el::p().class("review-status").text(&recipe.rustc_version)),
        );
    }
    let mut links = el::div()
        .class("hero-links")
        .child(
            el::a()
                .class("primary-link")
                .href(format!("{source}/SKILL.md"))
                .text("Read the skill ↗"),
        )
        .child(
            el::a()
                .class("text-link")
                .href(site.url("subscribe.html"))
                .text("Install with Symposium →"),
        );
    if let Some(tag) = &record.entry.bundle_release {
        links = links.child(
            el::a()
                .class("text-link")
                .href(format!("{REPO}/releases/tag/{tag}"))
                .text("Full bundle + checksum ↗"),
        );
    }
    site.page(&format!("{} {}", p.release.name, p.release.version), &record.entry.summary, "release",
        el::article().class("release-page prose")
            .child(el::a().href(site.url("")).text("← All releases"))
            .child(el::p().class("eyebrow").text("EXACT RELEASE GUIDANCE"))
            .child(el::h1().text(format!("{} {}", p.release.name, p.release.version)))
            .child(el::p().class("release-summary").text(&record.entry.summary))
            .child(links)
            .child(el::h2().text("Recipes and verification"))
            .child(recipes)
            .child(el::p().text(review_label(record)))
            .child(el::p().text("Compilation records apply to the recorded recipe inputs and lockfiles. They do not establish runtime behavior. The bundle schema does not record the verification platform. See the skill for collection warnings, citations, features, and coverage limits."))
            .child(el::h2().text("Trace the evidence"))
            .child(el::p().child(el::a().href(format!("{source}/references/provenance.json")).text("Inspect release provenance, source hashes, and lockfile records ↗")))
            .child(el::p().child(el::a().href(format!("https://crates.io/crates/{}/{}", p.release.name, p.release.version)).text("Published crate ↗")))
            .child(el::p().child(el::a().href(p.release.repository.clone().unwrap_or_else(|| format!("https://crates.io/crates/{}", p.release.name))).text("Upstream project ↗")))
            .child(el::h2().text("Found something to improve?"))
            .child(el::p().child(el::a().href(site.url("subscribe.html#4-try-a-real-task")).text("Try this guidance on a real task"))
                .text(", then ").child(el::a().href(format!("{REPO}/issues/new?template=usage-feedback.yml")).text("share first or repeat use"))
                .text(". Tell us whether it installed, whether your agent consulted it, and how the task went."))
            .child(el::p().child(el::a().href(format!("{REPO}/issues/new?template=correction.yml")).text("Report a correction"))
                .text(" with the recipe or finding ID and a minimal reproduction."))
            .child(el::p().class("review-status").text("Guidance generated and checked with ")
                .child(el::a().href("https://mcaveniathor.github.io/symposiast/").text("Symposiast"))
                .text(". See the provenance for this entry’s recorded analysis and review status.")))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let output = args.next().unwrap_or_else(|| "dist".into());
    let base = match args.next().as_deref() {
        None => "/symposiast-registry/".to_owned(),
        Some("--base") => args.next().ok_or("--base requires a path")?,
        Some(_) => {
            return Err(
                "Usage: symposiast-registry-site [OUTPUT] [--base /symposiast-registry/]".into(),
            );
        }
    };
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let site = Site::new(normalize_base(&base)?);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let records = load(root)?;
    let out = Path::new(&output);
    fs::create_dir_all(out.join("assets"))?;
    fs::write(out.join("index.html"), catalog(&site, &records))?;
    for record in &records {
        fs::write(
            out.join(format!("{}.html", record.entry.entry)),
            detail(&site, record),
        )?;
    }
    for (slug, title, description) in DOCUMENTS {
        let path = if slug == "contributing" {
            root.join("CONTRIBUTING.md")
        } else {
            root.join(format!("site/content/{slug}.md"))
        };
        fs::write(
            out.join(format!("{slug}.html")),
            markdown::page(&site, slug, title, description, &fs::read_to_string(path)?),
        )?;
    }
    fs::write(
        out.join("404.html"),
        site.page(
            "Page not found",
            "Return to the registry catalog.",
            "404",
            el::section()
                .class("not-found")
                .child(el::h1().text("A missing reference."))
                .child(el::a().href(site.url("")).text("Return to the registry →")),
        ),
    )?;
    for url in site.bundle.paths() {
        let path = out.join(url.strip_prefix(&site.base).ok_or("asset outside base")?);
        fs::create_dir_all(path.parent().unwrap())?;
        fs::write(path, &site.bundle.get(&url).unwrap().bytes)?;
    }
    fs::copy(root.join("site/assets/site.js"), out.join("assets/site.js"))?;
    fs::copy(
        root.join("site/assets/favicon.svg"),
        out.join("assets/favicon.svg"),
    )?;
    fs::write(out.join(".nojekyll"), "")?;
    println!(
        "Built {} release pages and registry guides in {}",
        records.len(),
        out.display()
    );
    Ok(())
}
