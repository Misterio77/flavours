use anyhow::{Context, Result};
use serde::Deserialize;

/// Structure that represents a scheme
#[derive(Debug, Default)]
pub struct Scheme {
    pub slug: String,
    pub name: String,
    pub author: String,
    pub colors: [SchemeColor; 16],
}

/// Structure that represents a single color item in a scheme
#[derive(Debug, Default)]
pub struct SchemeColor {
    pub id: u8,
    pub color: String,
}

/// Structure for raw scheme
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SchemeFile {
    scheme: String,
    author: String,
    base00: String,
    base01: String,
    base02: String,
    base03: String,
    base04: String,
    base05: String,
    base06: String,
    base07: String,
    base08: String,
    base09: String,
    base0A: String,
    base0B: String,
    base0C: String,
    base0D: String,
    base0E: String,
    base0F: String,
}

impl Scheme {
    /// Creates a scheme from a YAML string
    pub fn from_str(contents: &str, slug: &str) -> Result<Scheme> {
        let scheme_file: SchemeFile = serde_yaml::from_str(contents).with_context(|| {
            format!(
                "Couldn't parse scheme {}. Check if it's syntatically correct.",
                slug
            )
        })?;
        let mut scheme = Scheme::default();
        scheme.slug = String::from(slug);
        scheme.name = scheme_file.scheme;
        scheme.author = scheme_file.author;

        for (color, id) in [
            (&scheme_file.base00, 0x00),
            (&scheme_file.base01, 0x01),
            (&scheme_file.base02, 0x02),
            (&scheme_file.base03, 0x03),
            (&scheme_file.base04, 0x04),
            (&scheme_file.base05, 0x05),
            (&scheme_file.base06, 0x06),
            (&scheme_file.base07, 0x07),
            (&scheme_file.base08, 0x08),
            (&scheme_file.base09, 0x09),
            (&scheme_file.base0A, 0x0A),
            (&scheme_file.base0B, 0x0B),
            (&scheme_file.base0C, 0x0C),
            (&scheme_file.base0D, 0x0D),
            (&scheme_file.base0E, 0x0E),
            (&scheme_file.base0F, 0x0F),
        ].iter() {
            scheme.colors[*id as usize].color = color.to_string();
            scheme.colors[*id as usize].id = *id as u8;
        }

        Ok(scheme)
    }
}
