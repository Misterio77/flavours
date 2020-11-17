use anyhow::{Context, Result};
use serde::Deserialize;

/// Structure for configuration
#[derive(Deserialize, Debug)]
pub struct Config {
    pub item: Option<Vec<ConfigItem>>,
}

/// Structure for configuration apply items
#[derive(Deserialize, Debug)]
pub struct ConfigItem {
    pub file: String,
    pub template: String,
    pub subtemplate: Option<String>,
    pub hook: Option<String>,
    pub rewrite: Option<bool>,
    pub start: Option<String>,
    pub end: Option<String>,
}

impl Config {
    /// Parse a TOML str into a Config struct
    pub fn from_str(contents: &str) -> Result<Config> {
        toml::from_str(&contents)
            .context("Couldn't parse configuration file. Check if it's syntatically correct")
    }
}
