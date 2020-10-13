use std::fs;
use std::path;
use std::process;
use std::str;
use std::thread;

use anyhow::{anyhow, Context, Result};
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    item: Option<Vec<ConfigItem>>,
}

/// Structure for configuration items
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

/// Structure for schemes
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

/// Picks a random path, from given vec
/// * `values` - Vec with paths
fn random(values: Vec<path::PathBuf>) -> Result<path::PathBuf> {
    let chosen = values.choose(&mut rand::thread_rng()).ok_or_else(|| {
        anyhow!(
            "Scheme not found. Check if it exists, or run update schemes if you didn't already."
        )
    })?;
    Ok(chosen.to_path_buf())
}

/// Runs hook commands
///
/// * `command` - Command string to execute
/// * `verbose` - Should we be verbose?
fn run_hook(command: &str, verbose: bool) -> Result<()> {
    if verbose && command != "" {
        println!("running {}", command);
    }
    let command_vec = shell_words::split(command)?;

    if !command_vec.is_empty() {
        if command_vec.len() == 1 {
            process::Command::new(&command_vec[0])
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .status()
                .with_context(|| format!("Couldn't run hook '{}'", command))?;
        } else {
            process::Command::new(&command_vec[0])
                .args(&command_vec[1..])
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .status()
                .with_context(|| format!("Couldn't run hook '{}'", command))?;
        }
    }

    Ok(())
}

/// Replace with delimiter lines
///
/// In a string, removes everything from one line to another, and puts the built template in place
///
/// * `file_content` - String with lines to be replaced
/// * `start` - Where to start replacing
/// * `end` - Where to stop replacing
/// * `built_template` - Built template to be injected
fn replace_delimiter(
    file_content: &str,
    start: &str,
    end: &str,
    built_template: &str,
) -> Result<String> {
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

/// Build a template
///
/// Given template base, scheme and scheme slug, builds the template and returns it
///
/// * `template_base` - Template base string
/// * `scheme` - Scheme structure
/// * `scheme_slug` - Scheme slug
fn build_template(template_base: String, scheme: &Scheme, scheme_slug: &str) -> Result<String> {
    let mut built_template = template_base;
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
        (&scheme.base0F, "base0F"),
    ]
    .iter()
    {
        let rgb = hex::decode(hex)?;
        built_template = built_template
            .replace(
                //hex
                &format!("{{{{{}-hex}}}}", name),
                hex,
            )
            .replace(
                //hex-r
                &format!("{{{{{}-hex-r}}}}", name),
                &hex[0..2],
            )
            .replace(
                //hex-g
                &format!("{{{{{}-hex-g}}}}", name),
                &hex[2..4],
            )
            .replace(
                //hex-b
                &format!("{{{{{}-hex-b}}}}", name),
                &hex[4..6],
            )
            .replace(
                //hex-bgr
                &format!("{{{{{}-hex-bgr}}}}", name),
                &format!("{}{}{}", &hex[4..6], &hex[2..4], &hex[0..2]),
            )
            .replace(
                //rgb-r
                &format!("{{{{{}-rgb-r}}}}", name),
                &format!("{}", rgb[0]),
            )
            .replace(
                //rgb-g
                &format!("{{{{{}-rgb-g}}}}", name),
                &format!("{}", rgb[1]),
            )
            .replace(
                //rgb-b
                &format!("{{{{{}-rgb-b}}}}", name),
                &format!("{}", rgb[2]),
            )
            .replace(
                //dec-r
                &format!("{{{{{}-dec-r}}}}", name),
                &format!("{:.2}", (rgb[0] as f64) / (255_f64)),
            )
            .replace(
                //dec-g
                &format!("{{{{{}-dec-g}}}}", name),
                &format!("{:.2}", (rgb[1] as f64) / (255_f64)),
            )
            .replace(
                //dec-b
                &format!("{{{{{}-dec-b}}}}", name),
                &format!("{:.2}", (rgb[2] as f64) / (255_f64)),
            )
    }
    Ok(built_template)
}

/// Apply function
///
/// * `patterns` - Which patterns the user specified
/// * `base_dir` - Flavours base directory
/// * `config_path` - Flavours configuration path
/// * `verbose` - Should we be verbose?
pub fn apply(
    patterns: Vec<&str>,
    base_dir: &path::Path,
    config_path: &path::Path,
    verbose: bool,
) -> Result<()> {
    //Find schemes that match given patterns
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find::find(pattern, &base_dir.join("base16").join("schemes"))?;

        for found_scheme in found_schemes {
            schemes.push(found_scheme);
        }
    }
    //Sort and remove duplicates
    schemes.sort();
    schemes.dedup();

    //Get random scheme
    let scheme_file = random(schemes)?;
    let scheme_slug = scheme_file
        .file_stem()
        .ok_or_else(|| anyhow!("Couldn't get scheme name."))?
        .to_str()
        .ok_or_else(|| anyhow!("Couldn't convert scheme file name."))?;

    //Read chosen scheme, store its data
    let scheme: Scheme = serde_yaml::from_str(
        &fs::read_to_string(&scheme_file)
            .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?,
    )
    .with_context(|| {
        format!(
            "Couldn't parse scheme {}. Check if it's syntatically correct.",
            scheme_slug
        )
    })?;

    if verbose {
        println!(
            "Using scheme: {} ({}), by {}",
            scheme.scheme, scheme_slug, scheme.author
        );
        println!();
    }

    //Check if config file exists
    if !config_path.exists() {
        eprintln!("Config {:?} doesn't exist, creating", config_path);
        let default_content = match fs::read_to_string(path::Path::new("/etc/flavours.conf")) {
            Ok(content) => content,
            Err(_) => String::from(""),
        };
        let config_path_parent = config_path
            .parent()
            .with_context(|| format!("Couldn't get parent directory of {:?}", config_path))?;

        fs::create_dir_all(config_path_parent).with_context(|| {
            format!(
                "Couldn't create configuration file parent directory {:?}",
                config_path_parent
            )
        })?;
        fs::write(config_path, default_content)
            .with_context(|| format!("Couldn't create configuration file at {:?}", config_path))?;
    }

    let config_contents = fs::read_to_string(config_path)
        .with_context(|| format!("Couldn't read configuration file {:?}.", config_path))?;

    let config_parsed: Config = toml::from_str(&config_contents).with_context(|| {
        "Couldn't parse configuration file. Check if it's syntatically correct."
    })?;

    let items = config_parsed.item.ok_or_else(||anyhow!("Couldn't get items from config file. Check the default file or github for config examples."))?;

    let mut hooks = Vec::new();

    //Iterate configurated entries (templates)
    for item in items {
        //File to write
        let file = path::Path::new(&shellexpand::full(&item.file)?.to_string())
            .canonicalize()
            .with_context(|| format!("Invalid file to write: {}.", item.file))?;
        //Template name
        let template = item.template;
        //Subtemplate name
        let subtemplate = match item.subtemplate {
            Some(value) => value,
            None => String::from("default"),
        };
        //Hook command
        let hook = match item.hook {
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
        }
        .trim()
        .to_lowercase();
        //Replace end delimiter
        let end = match item.end {
            Some(value) => value,
            None => String::from("# End flavours"),
        }
        .trim()
        .to_lowercase();

        //(sub)template file path
        let subtemplate_file = &base_dir
            .join("base16")
            .join("templates")
            .join(&template)
            .join("templates")
            .join(format!("{}.mustache", subtemplate));

        //Template content
        let template_content = fs::read_to_string(subtemplate_file)
                       .with_context(||format!("Couldn't read template {}/{} at {:?}. Check if the correct template/subtemplate was specified, and run the update templates command if you didn't already.", template, subtemplate, subtemplate_file))?;

        //Template with correct colors
        let built_template = build_template(template_content, &scheme, scheme_slug)?;

        //Rewrite file with built template
        if rewrite {
            fs::write(&file, built_template)
                .with_context(|| format!("Couldn't write to file {:?}.", file))?;

            if verbose {
                println!("Wrote {}/{} on: {:?}", template, subtemplate, file)
            }
        } else {
            //Or replace with delimiters
            let file_content = fs::read_to_string(&file)?;
            match replace_delimiter(&file_content, &start, &end, &built_template) {
                Ok(content) => fs::write(&file, content)
                    .with_context(|| format!("Couldn't write to file {:?}", file))?,
                Err(error) => eprintln!("Couldn't replace lines in {:?}: {}", file, error),
            }
            if verbose {
                println!("Wrote {}/{} on {:?}", template, subtemplate, file);
            }
        }
        hooks.push(thread::spawn(move || run_hook(&hook, verbose)));
    }

    let last_scheme_file = &base_dir.join("lastscheme");
    fs::write(&last_scheme_file, scheme_slug)
        .with_context(|| "Couldn't update applied scheme name")?;

    while !hooks.is_empty() {
        hooks
            .pop()
            .ok_or_else(|| anyhow!("Couldn't pop hooks."))?
            .join()
            .unwrap()?;
    }

    if verbose {
        println!("Successfully applied {}", scheme_slug);
    }
    Ok(())
}
