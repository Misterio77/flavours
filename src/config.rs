use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Structure for configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub shell: Option<String>,
    pub schemes: Option<String>,
    pub templates: Option<String>,
    pub extra_scheme: Option<Vec<ExtraSource>>,
    pub extra_template: Option<Vec<ExtraSource>>,
    pub item: Option<Vec<ConfigItem>>,
    pub items: Option<Vec<ConfigItem>>,
}

/// Structure for configuration extra sources
#[derive(Deserialize, Debug)]
pub struct ExtraSource{
    pub name: String,
    pub source: String,
}

/// Structure for configuration apply items
#[derive(Deserialize, Debug)]
pub struct ConfigItem {
    pub file: String,
    pub template: String,
    pub subtemplate: Option<String>,
    pub hook: Option<String>,
    pub rewrite: Option<bool>,
    pub light: Option<bool>,
    pub start: Option<String>,
    pub end: Option<String>,
}

impl Config {
    /// Parse a TOML str into a Config struct
    pub fn read(contents: &str, path: &Path) -> Result<Config> {
        toml::from_str(contents)
            .context(format!("Couldn't parse flavours configuration file ({:?}). Check if it's syntatically correct", path))
    }
}
