use serde::Deserialize;
use anyhow::{Context, Result};

/// Structure for schemes
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Scheme {
    pub scheme: String,
    pub author: String,
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    pub base0A: String,
    pub base0B: String,
    pub base0C: String,
    pub base0D: String,
    pub base0E: String,
    pub base0F: String,
}
/// Parses a Scheme string
/// * `input` - Scheme yaml string
/// * `scheme_slug` - Scheme slug name
pub fn parse_scheme(input: &str, scheme_slug: &str) -> Result<Scheme> {
    let scheme: Scheme = serde_yaml::from_str(input).with_context(|| {
        format!(
            "Couldn't parse scheme {}. Check if it's syntatically correct.",
            scheme_slug
        )
    })?;
    Ok(scheme)
}
