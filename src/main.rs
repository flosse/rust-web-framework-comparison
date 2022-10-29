use anyhow::Result;
use std::{fs::File, io::Write};
use tera::{Context, Tera};
use url::{Host, Url};

mod data;
mod table;

use data::*;

fn main() -> Result<()> {
    let data = read_data()?;

    // --- frontends --- //

    let mut active_frontends = data
        .frontend
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false))
        .collect::<Vec<_>>();
    active_frontends.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = frontends_to_table(&active_frontends);
    let frontend_frameworks = table::to_markdown(&table);

    let mut outdated_frontends = data
        .frontend
        .iter()
        .filter(|f| f.outdated == Some(true))
        .collect::<Vec<_>>();
    outdated_frontends.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = frontends_to_table(&outdated_frontends);
    let outdated_frontend_frameworks = table::to_markdown(&table);

    // --- templating --- //

    let mut active_templating = data
        .template
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false))
        .collect::<Vec<_>>();
    active_templating.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = templates_to_table(&active_templating);
    let templating = table::to_markdown(&table);

    // --- server --- //

    let mut server = data
        .server
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false))
        .filter(|f| f.low_level == None || f.low_level == Some(false))
        .collect::<Vec<_>>();
    server.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = server_to_table(&server);
    let server = table::to_markdown(&table);

    // --- low level server --- //

    let mut low_level_server = data
        .server
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false))
        .filter(|f| f.low_level == Some(true))
        .collect::<Vec<_>>();
    low_level_server.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = server_to_table(&low_level_server);
    let low_level_server = table::to_markdown(&table);

    // --- outdated server --- //

    let mut outdated_server = data
        .server
        .iter()
        .filter(|f| f.outdated == Some(true))
        .collect::<Vec<_>>();
    outdated_server.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = server_to_table(&outdated_server);
    let outdated_server = table::to_markdown(&table);

    // --- web sockets --- //

    let mut websocket = data
        .websocket
        .iter()
        .filter(|f| f.outdated.is_none() || f.outdated == Some(false))
        .collect::<Vec<_>>();
    websocket.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    let table = websocket_to_table(&websocket);
    let websocket = table::to_markdown(&table);

    // --- render README --- //

    let tera = Tera::new("*.tmpl")?;
    let mut context = Context::new();
    context.insert("frontend_frameworks", &frontend_frameworks);
    context.insert(
        "outdated_frontend_frameworks",
        &outdated_frontend_frameworks,
    );
    context.insert("templating", &templating);
    context.insert("server", &server);
    context.insert("outdated_server", &outdated_server);
    context.insert("low_level_server", &low_level_server);
    context.insert("websocket", &websocket);

    let readme = tera.render("README.tmpl", &context)?;
    let mut file = File::create("README.md")?;
    file.write_all(readme.as_bytes())?;

    Ok(())
}

fn frontends_to_table(frontends: &[&Frontend]) -> table::Table {
    let mut rows = vec![vec![
        "Name".to_string(),
        "Stars".to_string(),
        "Contributors".to_string(),
        "Activity".to_string(),
        "Virtual DOM".to_string(),
        "SSR".to_string(),
        "Rendering".to_string(),
        "Architecture".to_string(),
        "Repo".to_string(),
        "Docs".to_string(),
        "License".to_string(),
        "Version".to_string(),
    ]];

    for f in frontends {
        let Frontend {
            name,
            repo,
            homepage,
            crates_io,
            vdom,
            ssr,
            rendering,
            architecture,
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

        let vdom = vdom.map(bool_to_str).unwrap_or_default().to_string();
        let ssr = ssr.map(bool_to_str).unwrap_or_default().to_string();
        let rendering = rendering.map(|r| r.to_string()).unwrap_or_default();
        let architecture = architecture
            .map(|r| {
                let url = r.info_url().to_string();
                let label = r.to_string();
                format!("[{label}]({url})")
            })
            .unwrap_or_default();

        rows.push(vec![
            name,
            stars,
            contributors,
            activity,
            vdom,
            ssr,
            rendering,
            architecture,
            repo,
            docs,
            license,
            version,
        ]);
    }
    rows
}

const fn bool_to_str(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}

const fn opt_bool_to_str(b: Option<bool>) -> &'static str {
    if let Some(b) = b {
        bool_to_str(b)
    } else {
        ""
    }
}

fn templates_to_table<'a>(templates: &[&Template]) -> table::Table {
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

fn server_to_table<'a>(servers: &[&Server]) -> table::Table {
    let mut rows = vec![vec![
        "Name".to_string(),
        "Stars".to_string(),
        "Contributors".to_string(),
        "Activity".to_string(),
        "Repo".to_string(),
        "Docs".to_string(),
        "License".to_string(),
        "Version".to_string(),
        "Async".to_string(),
        "HTTPS".to_string(),
        "HTTP/2".to_string(),
        "Base".to_string(),
        "Client".to_string(),
    ]];

    for s in servers {
        let Server {
            name,
            repo,
            homepage,
            crates_io,
            r#async,
            https,
            http2,
            base,
            client,
            ..
        } = s;

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

        let base = base.as_ref().map(|b| b.to_string()).unwrap_or_default();

        rows.push(vec![
            name,
            stars,
            contributors,
            activity,
            repo,
            docs,
            license,
            version,
            opt_bool_to_str(*r#async).into(),
            opt_bool_to_str(*https).into(),
            opt_bool_to_str(*http2).into(),
            base,
            opt_bool_to_str(*client).into(),
        ]);
    }
    rows
}

fn websocket_to_table<'a>(websocket: &[&WebSocket]) -> table::Table {
    let mut rows = vec![vec![
        "Name".to_string(),
        "Repo".to_string(),
        "Docs".to_string(),
        "License".to_string(),
        "Version".to_string(),
        "Stars".to_string(),
        "Contributors".to_string(),
        "Activity".to_string(),
        "Client".to_string(),
        "Server".to_string(),
        "Async".to_string(),
    ]];

    for s in websocket {
        let WebSocket {
            name,
            repo,
            homepage,
            crates_io,
            r#async,
            client,
            server,
            ..
        } = s;

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
            bool_to_str(*client).into(),
            bool_to_str(*server).into(),
            bool_to_str(*r#async).into(),
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
