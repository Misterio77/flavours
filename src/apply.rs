use std::path;
use std::fs;

use anyhow::{anyhow, Result, Context};
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    item: Option<Vec<ConfigItem>>,
}

#[derive(Deserialize, Debug)]
struct ConfigItem {
    file: String,
    template: String,
    subtemplate: Option<String>,
    hook: Option<String>,
    rewrite: Option<bool>,
    start: Option<String>,
    end: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Scheme {
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

#[path = "find.rs"]
mod find;

fn random(values: Vec<path::PathBuf>) -> Result<path::PathBuf> {
    let chosen = values
        .choose(&mut rand::thread_rng())
        .ok_or(
            anyhow!("Scheme not found")
        )?;
    Ok(chosen.to_path_buf())
}

pub fn apply(arguments: &clap::ArgMatches, base_dir: &path::Path, config_path: &path::Path, verbose: bool) -> Result<()> {
    //Get search patterns
    let patterns = match arguments.values_of("pattern") {
        Some(schemes) => schemes.collect(),
        //If none is supplied, defaults to wildcard
        None => vec!["*"],
    };

    //Find schemes that match given patterns
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find::find(
            pattern,
            &base_dir.join("base16").join("schemes")
        )?;

        for found_scheme in found_schemes {
            schemes.push(found_scheme);
        }
    }
    //Sort and remove duplicates
    schemes.sort();
    schemes.dedup();

    //Get random scheme
    let chosen = random(schemes)?;

    //Read chosen scheme, store its data
    let scheme: Scheme = serde_yaml::from_str(&fs::read_to_string(&chosen)?).with_context(|| format!("Couldn't read scheme at {:?}. Verify if it contains all needed values and try again.", chosen))?;
    if verbose {
        println!("Using scheme: {} ({:?}), by {}", 
                 scheme.scheme, 
                 chosen.file_stem()
                 .ok_or(anyhow!("Couldn't get scheme name"))?,
                 scheme.author);
    }

    //Read configuration
    let configuration: Config = toml::from_str(&fs::read_to_string(config_path)?).with_context(|| "Couldn't read configuration file. Check if all items are valid and include (at least) a file and template keys.")?;
    let items = configuration.item.ok_or(anyhow!("Error reading items from file. Try adding some."))?;

    for item in items {
        let file = item.file;
        let template = item.template;

        let subtemplate = match item.subtemplate {
            Some(value) => value,
            None => String::from("default"),
        };
        let hook = match item.hook {
            Some(value) => value,
            None => String::from(""),
        };
        let rewrite = match item.rewrite {
            Some(value) => value,
            None => false,
        };
        let start = match item.start {
            Some(value) => value,
            None => String::from("# Start flavours"),
        };
        let end = match item.end {
            Some(value) => value,
            None => String::from("# End flavours"),
        };

        let subtemplate_file = &base_dir
                               .join("base16")
                               .join("templates")
                               .join(&template)
                               .join("templates")
                               .join(format!("{}.mustache", subtemplate));

        let mut template_content = fs::read_to_string(subtemplate_file)
                       .with_context(|| format!("Couldn't read template {}/{} at {:?}. Check if the correct template/subtemplate was specified, and run the update command if you didn't already.", template, subtemplate, subtemplate_file))?;

        template_content = template_content
            .replace("{{scheme-name}}", &scheme.scheme);
        template_content = template_content
            .replace("{{scheme-author}}", &scheme.author);
        for (color, name) in [
            (&scheme.base00, "base00"),
            (&scheme.base01, "base01"),
            (&scheme.base02, "base02"),
            (&scheme.base03, "base03"),
            (&scheme.base04, "base04"),
            (&scheme.base05, "base05"),
            (&scheme.base06, "base06"),
            (&scheme.base07, "base07"),
            (&scheme.base08, "base08"),
            (&scheme.base09, "base09"),
            (&scheme.base0A, "base0A"),
            (&scheme.base0B, "base0B"),
            (&scheme.base0C, "base0C"),
            (&scheme.base0D, "base0D"),
            (&scheme.base0E, "base0E"),
            (&scheme.base0F, "base0F")
        ].iter() {
            template_content = template_content
                .replace(
                    &format!("{{{{{}-hex}}}}", name),
                    color);
        }



        println!("=============");
        println!("Template {}/{}:", template, subtemplate);
        println!("{}", template_content)

    }
    Ok(())
}
