use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Structure for schemes
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
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

/// Turns a Scheme struct into a yaml string
/// * `input` - Scheme struct
/// * `scheme_slug` - Scheme slug name
pub fn serialize_scheme(input: &Scheme, scheme_slug: &str) -> Result<String> {
    let output = serde_yaml::to_string(&input)
        .with_context(|| format!("Couldn't serialize scheme {} into YAML.", scheme_slug))?;

    Ok(output)
}
