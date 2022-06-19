use anyhow::Result;
use serde::Deserialize;
use std::{fs::File, io::Read};
use url::Url;

pub fn read_data() -> Result<Data> {
    let mut file = File::open("data.toml")?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;
    let data = toml::from_str(&toml_string)?;
    Ok(data)
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub frontend: Vec<Frontend>,
    pub template: Vec<Template>,
    pub server: Vec<Server>,
    pub websocket: Vec<WebSocket>,
}

#[derive(Debug, Deserialize)]
pub struct Frontend {
    pub name: String,
    pub repo: Url,
    pub homepage: Option<Url>,
    #[serde(rename = "crates-io")]
    pub crates_io: Option<String>,
    pub vdom: Option<bool>,
    pub ssr: Option<bool>,
    pub outdated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub name: String,
    pub repo: Url,
    pub homepage: Option<Url>,
    #[serde(rename = "crates-io")]
    pub crates_io: Option<String>,
    pub outdated: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub repo: Url,
    pub homepage: Option<Url>,
    #[serde(rename = "crates-io")]
    pub crates_io: Option<String>,
    pub outdated: Option<bool>,
    #[serde(rename = "low-level")]
    pub low_level: Option<bool>,
    pub r#async: bool,
    pub http2: bool,
    pub https: bool,
    pub client: bool,
}

#[derive(Debug, Deserialize)]
pub struct WebSocket {
    pub name: String,
    pub repo: Url,
    pub homepage: Option<Url>,
    #[serde(rename = "crates-io")]
    pub crates_io: Option<String>,
    pub outdated: Option<bool>,
    pub r#async: bool,
    pub client: bool,
    pub server: bool,
}
