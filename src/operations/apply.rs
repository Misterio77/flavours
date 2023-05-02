use anyhow::{anyhow, Context, Result};
use base16_color_scheme::Scheme;
use rand::seq::SliceRandom;
use std::fs;
use std::io::{self, Read};
use std::path;
use std::process;
use std::str;
use std::thread;

use crate::config::Config;
use crate::find::{find_schemes, find_template};
use crate::operations::build::build_template;

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
fn run_hook(command: Option<String>, shell: &str, verbose: bool) -> Result<()> {
    if let Some(command) = command {
        let full_command = shell.replace("{}", &command);
        if verbose {
            println!("running {}", full_command);
        }
        let command_vec = shell_words::split(&full_command)?;

        if command_vec.len() == 1 {
            process::Command::new(&command_vec[0])
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .status()
                .with_context(|| format!("Couldn't run hook '{}'", full_command))?;
        } else {
            process::Command::new(&command_vec[0])
                .args(&command_vec[1..])
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .status()
                .with_context(|| format!("Couldn't run hook '{}'", full_command))?;
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
                changed_content.push_str(built_template);
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

/// Apply function
///
/// * `patterns` - Which patterns the user specified
/// * `base_dir` - Flavours base directory
/// * `config_path` - Flavours configuration path
/// * `light` - Don't run hooks marked as non-lightweight
/// * `from_stdin` - Read scheme from stdin?
/// * `verbose` - Should we be verbose?
pub fn apply(
    patterns: Vec<&str>,
    base_dir: &path::Path,
    config_dir: &path::Path,
    config_path: &path::Path,
    light_mode: bool,
    from_stdin: bool,
    verbose: bool,
) -> Result<()> {
    let (scheme_contents, scheme_slug) = if from_stdin {
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;
        (buffer, String::from("generated"))
    } else {
        //Find schemes that match given patterns
        let mut schemes = Vec::new();
        for pattern in patterns {
            let found_schemes = find_schemes(pattern, base_dir, config_dir)?;

            for found_scheme in found_schemes {
                schemes.push(found_scheme);
            }
        }
        //Sort and remove duplicates
        schemes.sort();
        schemes.dedup();

        //Get random scheme
        let scheme_file = random(schemes)?;
        let scheme_slug: String = scheme_file
            .file_stem()
            .ok_or_else(|| anyhow!("Couldn't get scheme name."))?
            .to_str()
            .ok_or_else(|| anyhow!("Couldn't convert scheme file name."))?
            .into();

        //Read chosen scheme
        (
            fs::read_to_string(&scheme_file)
                .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?,
            scheme_slug,
        )
    };

    let mut scheme: Scheme = serde_yaml::from_str(&scheme_contents)?;
    scheme.slug = scheme_slug;

    if verbose {
        println!(
            "Using scheme: {} ({}), by {}",
            scheme.scheme, scheme.slug, scheme.author
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

    let config = Config::read(&config_contents, config_path)?;

    // If shell is present, check if it contains the placeholder
    let shell = config.shell.unwrap_or_else(|| "sh -c '{}'".into());

    if !shell.contains("{}") {
        // Hide {} in this error message from the formatting machinery in anyhow macro
        let msg = "The configured shell does not contain the required command placeholder '{}'. Check the default file or github for config examples.";
        return Err(anyhow!(msg));
    }

    let mut hooks = Vec::new();

    //Iterate configurated entries (templates)
    let items_legacy = config.item.unwrap_or_default();
    let mut items = config.items.unwrap_or_default();
    items.extend(items_legacy.into_iter());

    if items.is_empty() {
        return Err(anyhow!("Couldn't get items from config file. Check the default file or github for config examples."));
    }

    for item in items.iter() {
        //Template name
        let template = &item.template;
        //Subtemplate name
        let mut subtemplate = match &item.subtemplate {
            Some(value) => String::from(value),
            None => String::from("default"),
        };
        if subtemplate == "{scheme}" {
            let subtemplate_scheme = find_template(template, &scheme.name, base_dir, config_dir);
            subtemplate = match subtemplate_scheme {
                Ok(_value) => scheme.name.clone(),
                Err(_e) => String::from("default"),
            }
        };
        //Is the hook lightweight?
        let light = match &item.light {
            Some(value) => *value,
            None => true,
        };
        //Rewrite or replace
        let rewrite = match &item.rewrite {
            Some(value) => *value,
            None => false,
        };
        //Replace start delimiter
        let start = match &item.start {
            Some(value) => String::from(value),
            None => String::from("# Start flavours"),
        }
        .trim()
        .to_lowercase();
        //Replace end delimiter
        let end = match &item.end {
            Some(value) => String::from(value),
            None => String::from("# End flavours"),
        }
        .trim()
        .to_lowercase();

        let subtemplate_file = find_template(template, &subtemplate, base_dir, config_dir)
            .with_context(|| {
                format!(
                    "Failed to locate subtemplate file {}/{}",
                    template, subtemplate,
                )
            })?;

        //Template content
        let template_content = fs::read_to_string(&subtemplate_file)
                       .with_context(||format!("Couldn't read template {}/{} at {:?}. Check if the correct template/subtemplate was specified, and run the update templates command if you didn't already.", template, subtemplate, subtemplate_file))?;

        //Template with correct colors
        let built_template = build_template(&template_content, &scheme)
            .context("Couldn't replace placeholders. Check if all colors on the specified scheme file are valid (don't include a leading '#').")?;

        //File to write
        let file = shellexpand::full(&item.file)?.to_string();

        //Rewrite file with built template
        if rewrite {
            std::path::Path::new(&file)
                .parent()
                .and_then(|p| fs::create_dir_all(p).ok());
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

        let command = item.hook.clone();
        let shell = shell.clone();
        // Only add hook to queue if either:
        // - Not running on lightweight mode
        // - Hook is set as lightweight
        if !light_mode || light {
            hooks.push(thread::spawn(move || run_hook(command, &shell, verbose)));
        }
    }

    let last_scheme_file = &base_dir.join("lastscheme");
    fs::write(&last_scheme_file, &scheme.scheme_slug())
        .with_context(|| "Couldn't update applied scheme name")?;

    while !hooks.is_empty() {
        hooks
            .pop()
            .ok_or_else(|| anyhow!("Couldn't pop hooks."))?
            .join()
            .unwrap()?;
    }

    if verbose {
        println!("Successfully applied {}", &scheme.slug);
    }
    Ok(())
}
