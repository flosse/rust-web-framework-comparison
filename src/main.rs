use anyhow::Result;
use std::{fs::File, io::Write};
use tera::{Context, Tera};
use url::{Host, Url};

mod data;
mod table;

use data::*;

fn main() -> Result<()> {
    let data = read_data()?;
    let active_frontends = data
        .frontend
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false));
    let table = frontends_to_table(active_frontends);
    let frontend_frameworks = table::to_markdown(&table);

    let outdated_frontends = data.frontend.iter().filter(|f| f.outdated == Some(true));
    let table = frontends_to_table(outdated_frontends);
    let outdated_frontend_frameworks = table::to_markdown(&table);

    let active_templating = data
        .template
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false));
    let table = templates_to_table(active_templating);
    let templating = table::to_markdown(&table);

    let tera = Tera::new("*.tmpl")?;
    let mut context = Context::new();
    context.insert("frontend_frameworks", &frontend_frameworks);
    context.insert(
        "outdated_frontend_frameworks",
        &outdated_frontend_frameworks,
    );
    context.insert("templating", &templating);

    let readme = tera.render("README.tmpl", &context)?;
    let mut file = File::create("README.md")?;
    file.write_all(readme.as_bytes())?;

    Ok(())
}

fn frontends_to_table<'a>(frontends: impl Iterator<Item = &'a Frontend>) -> table::Table {
    let mut rows = vec![vec![
        "Name".to_string(),
        "Repo".to_string(),
        "Docs".to_string(),
        "License".to_string(),
        "Version".to_string(),
        "Stars".to_string(),
        "Contributors".to_string(),
        "Activity".to_string(),
        "Virtual DOM".to_string(),
        "SSR".to_string(),
    ]];

    for f in frontends {
        let Frontend {
            name,
            repo,
            homepage,
            crates_io,
            vdom,
            ssr,
            ..
        } = f;

        let RepoInfo {
            repo,
            stars,
            contributors,
            activity,
        } = repo_info(name, repo);

        let name = if let Some(hp) = homepage {
            format!("**[{name}]({hp})**")
        } else {
            format!("**{name}**")
        };

        let CratesIoInfo {
            license,
            docs,
            version,
        } = crates_io_info(&name, crates_io);

        let vdom = vdom
            .map(|yes| if yes { "yes" } else { "no" })
            .unwrap_or_default()
            .to_string();
        let ssr = ssr
            .map(|yes| if yes { "yes" } else { "no" })
            .unwrap_or_default()
            .to_string();

        rows.push(vec![
            name,
            repo,
            docs,
            license,
            version,
            stars,
            contributors,
            activity,
            vdom,
            ssr,
        ]);
    }
    rows
}

fn templates_to_table<'a>(templates: impl Iterator<Item = &'a Template>) -> table::Table {
    let mut rows = vec![vec![
        "Name".to_string(),
        "Repo".to_string(),
        "Docs".to_string(),
        "License".to_string(),
        "Version".to_string(),
        "Stars".to_string(),
        "Contributors".to_string(),
        "Activity".to_string(),
    ]];

    for t in templates {
        let Template {
            name,
            repo,
            homepage,
            crates_io,
            ..
        } = t;

        let RepoInfo {
            repo,
            stars,
            contributors,
            activity,
        } = repo_info(name, repo);

        let name = if let Some(hp) = homepage {
            format!("**[{name}]({hp})**")
        } else {
            format!("**{name}**")
        };

        let CratesIoInfo {
            license,
            docs,
            version,
        } = crates_io_info(&name, crates_io);

        rows.push(vec![
            name,
            repo,
            docs,
            license,
            version,
            stars,
            contributors,
            activity,
        ]);
    }
    rows
}

struct RepoInfo {
    repo: String,
    stars: String,
    contributors: String,
    activity: String,
}

fn repo_info(name: &str, repo: &Url) -> RepoInfo {
    let (repo, stars, contributors, activity) = match repo.host() {
        Some(Host::Domain("github.com")) => {
            let path = repo.path();
            let repo =
                format!("[![{name} repo](https://img.shields.io/badge/GitHub-git-blue)]({repo})");
            let stars =
                format!("![{name} stars](https://img.shields.io/github/stars{path}.svg?label=%20)");
            let contributors = format!("![{name} contributors](https://img.shields.io/github/contributors{path}.svg?label=%20)");
            let activity =  format!("![{name} activity](https://img.shields.io/github/commit-activity/y{path}.svg?label=%20)");
            (repo, stars, contributors, activity)
        }
        Some(Host::Domain("gitlab.com")) => (
            format!("[![Repo](https://img.shields.io/badge/GitLab-git-blue)]({repo})"),
            String::new(),
            String::new(),
            String::new(),
        ),
        _ => (
            format!("[Repo]({repo})"),
            String::new(),
            String::new(),
            String::new(),
        ),
    };
    RepoInfo {
        repo,
        stars,
        contributors,
        activity,
    }
}

#[derive(Default)]
struct CratesIoInfo {
    license: String,
    docs: String,
    version: String,
}

fn crates_io_info(name: &str, crates_io: &Option<String>) -> CratesIoInfo {
    if let Some(crate_name) = crates_io {
        let docs = format!("[![Docs](https://img.shields.io/badge/docs.rs-{crate_name}-green)](https://docs.rs/{crate_name}/)");
        let license = format!(
            "![{name} license](https://img.shields.io/crates/l/{crate_name}.svg?label=%20)"
        );
        let version = format!(
            "![{name} version](https://img.shields.io/crates/v/{crate_name}.svg?label=%20)"
        );
        CratesIoInfo {
            license,
            docs,
            version,
        }
    } else {
        CratesIoInfo::default()
    }
}
