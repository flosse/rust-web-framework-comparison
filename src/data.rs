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
}

#[derive(Debug, Deserialize)]
pub struct Frontend {
    pub name: String,
    pub repo: Url,
    pub homepage: Option<Url>,
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
    pub crates_io: Option<String>,
    pub outdated: Option<bool>,
}
