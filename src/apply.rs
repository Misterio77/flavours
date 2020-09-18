use std::path;
use std::fs;
use std::str;

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

#[allow(non_snake_case)]
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

fn replace_delimiter(file_content: &str, start: &str, end: &str, built_template: &str) -> Result<String> {
    let mut changed_content = String::new();

    let mut found_start = false;
    let mut found_end = false;

    let mut appended = false;

    for line in file_content.lines() {
        if found_start && !found_end {
            if !appended {
                changed_content.push_str(&built_template);
                appended = true;
            }
            if line.trim().to_lowercase().eq(&end) {
                changed_content.push_str(&format!("{}\n", line));
                found_end = true;
            }
        } else {
            changed_content.push_str(&format!("{}\n", line));
            if line.trim().to_lowercase().eq(&start) {
                found_start = true
            }
        }
    }
    if !found_start {
        Err(anyhow!("Couldn't find starting string."))
    } else if !found_end {
        Err(anyhow!("Couldn't find ending string."))
    } else {
        Ok(changed_content)
    }
}
 
fn build_template(template_base: String, scheme: &Scheme, scheme_slug: &str) -> Result<String> {
    let mut built_template = String::from(template_base);
    built_template = built_template
        .replace("{{scheme-name}}", &scheme.scheme)
        .replace("{{scheme-author}}", &scheme.author)
        .replace("{{scheme-slug}}", &scheme_slug);
    for (hex, name) in [
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
        let rgb = hex::decode(hex)?;
        built_template = built_template
            .replace( //hex
                &format!("{{{{{}-hex}}}}", name),
                hex)
            .replace( //hex-r
                &format!("{{{{{}-hex-r}}}}", name),
                &hex[0..2])
            .replace( //hex-g
                &format!("{{{{{}-hex-g}}}}", name),
                &hex[2..4])
            .replace( //hex-b
                &format!("{{{{{}-hex-b}}}}", name),
                &hex[4..6])
            .replace( //hex-bgr
                &format!("{{{{{}-hex-bgr}}}}", name),
                &format!("{}{}{}", &hex[4..6], &hex[2..4], &hex[0..2]))
            .replace( //rgb-r
                &format!("{{{{{}-rgb-r}}}}", name),
                &format!("{}", rgb[0]))
            .replace( //rgb-g
                &format!("{{{{{}-rgb-g}}}}", name),
                &format!("{}", rgb[1]))
            .replace( //rgb-b
                &format!("{{{{{}-rgb-b}}}}", name),
                &format!("{}", rgb[2]))
            .replace( //dec-r
                &format!("{{{{{}-dec-r}}}}", name),
                &format!("{:.2}", (rgb[0] as f64)/(255 as f64)))
            .replace( //dec-g
                &format!("{{{{{}-dec-g}}}}", name),
                &format!("{:.2}", (rgb[1] as f64)/(255 as f64)))
            .replace( //dec-b
                &format!("{{{{{}-dec-b}}}}", name),
                &format!("{:.2}", (rgb[2] as f64)/(255 as f64)))
    }
    Ok(built_template)
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
    let scheme_file = random(schemes)?;
    let scheme_slug = scheme_file.file_stem()
                      .ok_or(anyhow!("Couldn't get scheme name"))?
                      .to_str()
                      .ok_or(anyhow!("Couldn't convert scheme file name"))?;

    //Read chosen scheme, store its data
    let scheme: Scheme = serde_yaml::from_str(&fs::read_to_string(&scheme_file)?).with_context(|| format!("Couldn't read scheme at {:?}. Verify if it contains all needed values and try again.", scheme_file))?;

    if verbose {
        println!("Using scheme: {} ({}), by {}", 
                 scheme.scheme, 
                 scheme_slug,
                 scheme.author);
    }

    //Read configuration
    let configuration: Config = toml::from_str(&fs::read_to_string(config_path)?).with_context(|| "Couldn't read configuration file. Check if all items are valid and include (at least) a file and template keys.")?;
    let items = configuration.item.ok_or(anyhow!("Error reading items from file. Try adding some."))?;

    //Iterate configurated entries (templates)
    for item in items {
        //File to write
        let file = path::Path::new(
                &shellexpand::full(&item.file)?
                .to_string()
            )
            .canonicalize()
            .with_context(|| format!("Invalid file to write: {}", item.file))?;
        //Template name
        let template = item.template;
        //Subtemplate name
        let subtemplate = match item.subtemplate {
            Some(value) => value,
            None => String::from("default"),
        };
        //Hook command
        let _hook = match item.hook {
            Some(value) => value,
            None => String::from(""),
        };
        //Rewrite or replace
        let rewrite = match item.rewrite {
            Some(value) => value,
            None => false,
        };
        //Replace start delimiter
        let start = match item.start {
            Some(value) => value,
            None => String::from("# Start flavours"),
        }.trim().to_lowercase();
        //Replace end delimiter
        let end = match item.end {
            Some(value) => value,
            None => String::from("# End flavours"),
        }.trim().to_lowercase();

        //(sub)template file path
        let subtemplate_file = &base_dir
                               .join("base16")
                               .join("templates")
                               .join(&template)
                               .join("templates")
                               .join(format!("{}.mustache", subtemplate));

        //Template content
        let template_content = fs::read_to_string(subtemplate_file)
                       .with_context(|| format!("Couldn't read template {}/{} at {:?}. Check if the correct template/subtemplate was specified, and run the update command if you didn't already.", template, subtemplate, subtemplate_file))?;

        //Template with correct colors
        let built_template = build_template(template_content, &scheme, scheme_slug)?;

        //Rewrite file with built template
        if rewrite {
            fs::write(&file, built_template)
                .with_context(||
                format!("Couldn't write to file {:?}", file))?;

            if verbose { println!("Wrote {}/{} on: {:?}", template, subtemplate, file) }

        } else { //Or replace with delimiters
            let file_content = fs::read_to_string(&file)?;
            match replace_delimiter(&file_content, &start, &end, &built_template) {
                Ok(content) => {
                    fs::write(&file, content)
                        .with_context(||
                        format!("Couldn't write to file {:?}", file))?
                }
                Err(error) => {
                    eprintln!("Error writing to file {:?}: {}", file, error)
                }
            }
            if verbose {
                println!("Wrote {}/{} on {:?}",
                         template,
                         subtemplate,
                         file);
            }
        }

    }
    if verbose {
        println!("Successfully applied {}", scheme_slug);
    }
    let last_scheme_file = &base_dir.join("lastscheme");
    fs::write(&last_scheme_file, scheme_slug)
       .with_context(||
        format!("Couldn't update applied scheme name"))?;
    Ok(())
}
