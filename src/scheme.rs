use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fmt::Write;

/// Structure that represents a scheme
#[derive(Debug, Default)]
pub struct Scheme {
    pub slug: String,
    pub name: String,
    pub author: String,
    pub colors: VecDeque<String>,
}

/// Structure for raw scheme
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Default)]
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
    base10: Option<String>,
    base11: Option<String>,
    base12: Option<String>,
    base13: Option<String>,
    base14: Option<String>,
    base15: Option<String>,
    base16: Option<String>,
    base17: Option<String>,
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
        scheme.colors.push_back(scheme_file.base00.clone());
        scheme.colors.push_back(scheme_file.base01);
        scheme.colors.push_back(scheme_file.base02);
        scheme.colors.push_back(scheme_file.base03);
        scheme.colors.push_back(scheme_file.base04);
        scheme.colors.push_back(scheme_file.base05);
        scheme.colors.push_back(scheme_file.base06);
        scheme.colors.push_back(scheme_file.base07);
        scheme.colors.push_back(scheme_file.base08.clone());
        scheme.colors.push_back(scheme_file.base09);
        scheme.colors.push_back(scheme_file.base0A.clone());
        scheme.colors.push_back(scheme_file.base0B.clone());
        scheme.colors.push_back(scheme_file.base0C.clone());
        scheme.colors.push_back(scheme_file.base0D.clone());
        scheme.colors.push_back(scheme_file.base0E.clone());
        scheme.colors.push_back(scheme_file.base0F);
        scheme.colors.push_back(scheme_file.base10.unwrap_or(scheme_file.base00.clone()));
        scheme.colors.push_back(scheme_file.base11.unwrap_or(scheme_file.base00));
        scheme.colors.push_back(scheme_file.base12.unwrap_or(scheme_file.base08));
        scheme.colors.push_back(scheme_file.base13.unwrap_or(scheme_file.base0A));
        scheme.colors.push_back(scheme_file.base14.unwrap_or(scheme_file.base0B));
        scheme.colors.push_back(scheme_file.base15.unwrap_or(scheme_file.base0C));
        scheme.colors.push_back(scheme_file.base16.unwrap_or(scheme_file.base0D));
        scheme.colors.push_back(scheme_file.base17.unwrap_or(scheme_file.base0E));

        Ok(scheme)
    }
    pub fn to_string(&self) -> Result<String> {
        let mut string = String::new();

        writeln!(&mut string, "scheme: \"{}\"", self.name)?;
        writeln!(&mut string, "author: \"{}\"", self.author)?;

        for (i, color) in self.colors.iter().enumerate() {
            writeln!(&mut string, "base0{:X}: \"{}\"", i, color)?;
        }
        Ok(string)
    }
}
