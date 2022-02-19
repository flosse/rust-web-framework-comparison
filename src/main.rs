use anyhow::Result;
use pad::PadStr;
use serde::Deserialize;
use std::{
    cmp::max,
    fs::File,
    io::{Read, Write},
};
use tera::{Context, Tera};
use url::{Host, Url};

fn main() -> Result<()> {
    let data = read_data()?;

    let active_frontends = data
        .frontend
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false));
    let table = frontends_to_table(active_frontends);
    let frontend_frameworks = table_to_markdown(&table);

    let outdated_frontends = data.frontend.iter().filter(|f| f.outdated == Some(true));
    let table = frontends_to_table(outdated_frontends);
    let outdated_frontend_frameworks = table_to_markdown(&table);

    let tera = Tera::new("*.tmpl")?;
    let mut context = Context::new();
    context.insert("frontend_frameworks", &frontend_frameworks);
    context.insert(
        "outdated_frontend_frameworks",
        &outdated_frontend_frameworks,
    );
    let readme = tera.render("README.tmpl", &context)?;
    let mut file = File::create("README.md")?;
    file.write_all(readme.as_bytes())?;

    Ok(())
}

fn read_data() -> Result<Data> {
    let mut file = File::open("data.toml")?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;
    let data = toml::from_str(&toml_string)?;
    Ok(data)
}

#[derive(Debug, Deserialize)]
struct Data {
    frontend: Vec<Frontend>,
}

#[derive(Debug, Deserialize)]
struct Frontend {
    name: String,
    repo: Url,
    homepage: Option<Url>,
    crates_io: Option<String>,
    vdom: Option<bool>,
    ssr: Option<bool>,
    outdated: Option<bool>,
}

type Table = Vec<Row>;
type Row = Vec<Column>;
type Column = String;

fn frontends_to_table<'a>(frontends: impl Iterator<Item = &'a Frontend>) -> Table {
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

        let (repo, stars, contributors, activity) = match repo.host() {
            Some(Host::Domain("github.com")) => {
                let path = repo.path();
                let repo = format!(
                    "[![{name} repo](https://img.shields.io/badge/GitHub-git-blue)]({repo})"
                );
                let stars = format!(
                    "![{name} stars](https://img.shields.io/github/stars{path}.svg?label=%20)"
                );
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

        let name = if let Some(hp) = homepage {
            format!("**[{name}]({hp})**")
        } else {
            format!("**{name}**")
        };

        let (license, docs, version) = if let Some(crate_name) = crates_io {
            let docs = format!("[![Docs](https://img.shields.io/badge/docs.rs-{crate_name}-green)](https://docs.rs/{crate_name}/)");
            let license = format!(
                "![{name} license](https://img.shields.io/crates/l/{crate_name}.svg?label=%20)"
            );
            let version = format!(
                "![{name} version](https://img.shields.io/crates/v/{crate_name}.svg?label=%20)"
            );
            (license, docs, version)
        } else {
            (String::new(), String::new(), String::new())
        };

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

fn table_to_markdown(data: &[Row]) -> String {
    let lengths = data.iter().fold(vec![1; data[0].len()], |lens, row| {
        row.iter()
            .zip(lens)
            .map(|(s, len)| max(s.len(), len))
            .collect()
    });
    let rows = data
        .iter()
        .map(|row| {
            row.iter()
                .zip(&lengths)
                .map(|(s, len)| s.pad_to_width(*len))
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .collect::<Vec<_>>();
    let separator = lengths
        .iter()
        .map(|len| "-".repeat(*len))
        .collect::<Vec<_>>()
        .join("-|-");
    [
        format!("| {} |", rows[0]),
        format!("|-{}-|", separator),
        format!("| {} |", rows[1..].join(" |\n| ")),
    ]
    .join("\n")
}
