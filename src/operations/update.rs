use std::env::set_var;
use std::fs::{create_dir_all, remove_dir_all, write, File, read_to_string, OpenOptions};
use std::io::{self, BufRead, BufReader, prelude::*};
use std::path::Path;
use std::process::Command;
use std::thread::spawn;

use anyhow::{anyhow, Context, Result};
use crate::config::Config;

// nabbed from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

///Parses yml line containing name and repository link
///
///# Arguments
///* `line` - String containing both name and repository in format `name: repository`
fn parse_yml_line(line: &str) -> Result<(&str, String)> {
    let parsed: Vec<&str> = line.split(':').collect();
    let name = parsed[0];
    let repo = parsed[1..].join(":").split_whitespace().collect();
    Ok((name, repo))
}

///Writes schemes and templates repos information
///
///# Arguments
///* `s_repo` - String slice containing Schemes repository link
///* `t_repo` - String slice containing Schemes repository link
///* `file` - Path to sources file
fn write_sources(s_repo: &str, t_repo: &str, file: &Path) -> Result<()> {
    let mut file = File::create(file).with_context(|| format!("Couldn't open {:?}", file))?;
    write!(file, "schemes: {}\ntemplates: {}", s_repo, t_repo).with_context(|| format!("Couldn't open {:?}", file))?;

    Ok(())
}

///Gets schemes and repository links from file, falls back to default repos
///
///# Arguments
///* `file` - Path to sources file
fn get_sources(file: &Path, config: &Config) -> Result<(String, String)> {
    // Default repos
    let default_s_repo = "https://github.com/chriskempson/base16-schemes-source.git";
    let default_t_repo = "https://github.com/chriskempson/base16-templates-source.git";
    
    // config source indicators
    let use_config_sources = config.schemes.is_some();
    let use_config_templates = config.templates.is_some();

    // use available config sources, otherwise use defaults
    let (mut s_repo, mut t_repo) =
        match (config.schemes.as_ref(), config.templates.as_ref()) {
            (Some(s), Some(t)) => (String::from(s), String::from(t)),
            (Some(s), None) => (String::from(s), String::from(default_t_repo)),
            (None, Some(t)) => (String::from(default_s_repo), String::from(t)),
            (None, None) => (String::from(default_s_repo), String::from(default_t_repo)),
    };

    // Try to open file
    let sources_file = match File::open(file) {
        // Success
        Ok(contents) => contents,
        // Handle error once, so if file is not found it can be created
        Err(_) => {
            // Try to write config sources if they exist, otherwise use defaults
            write_sources(&s_repo, &t_repo, file)?;
            // Try to open it again, returns errors if unsucessful again
            File::open(file).with_context(|| format!("Couldn't access {:?}", file))?
        }
    };

    // Bufreader from file
    let reader = BufReader::new(sources_file);
    // Iterate lines
    for line in reader.lines() {
        let line = line?;
        // Get name and repo from line
        let (name, repo) = parse_yml_line(&line)?;
        // Store in correct variable
        // Only use sources.yaml sources if sources not specified in config
        if name == "schemes" && !use_config_sources {
            s_repo = repo;
        } else if name == "templates" && !use_config_templates {
            t_repo = repo;
        }
    }

    // Rewrite file using found repository, this is done to clean up errors on the file or insert default values
    write_sources(&s_repo, &t_repo, file)?;

    // Return repos
    Ok((s_repo, t_repo))
}
///Get name and repository vector from given list
///
///# Arguments
///* `file` - File with list
fn get_repo_list(file: &Path) -> Result<Vec<(String, String)>> {
    let sources_file = File::open(file).with_context(|| {
        format!(
            "Failed to read repository list {:?}. Try 'update lists' first?",
            file
        )
    })?;
    let mut result = Vec::new();

    let reader = BufReader::new(sources_file);

    for line in reader.lines() {
        let line = line?;
        let (name, repo) = parse_yml_line(&line)?;
        let first = name.chars().next();
        if first != Some('#') && first != None {
            result.push((name.into(), repo));
        }
    }

    Ok(result)
}

enum CloneType {
    Scheme,
    Template,
    List,
}

///Uses git to sparsely clone a repository, and then only checkout the .yml files and templates
///folder
///
///# Arguments
///* `path` - File path where repository should be cloned
///* `repo` - String slice containing link to repository
///* `verbose` - Boolean, tell git to be quiet if false
fn git_clone(path: &Path, repo: String, verbose: bool, clone_type: CloneType) -> Result<()> {
    // Remove directory, ignores errors
    let _ = remove_dir_all(path);
    set_var("GIT_TERMINAL_PROMPT", "0");

    // Try to clone into directory
    let command_clone = if verbose {
        Command::new("git")
            .arg("clone")
            .arg("-n")
            .arg(&repo)
            .arg(path)
            .arg("--depth")
            .arg("1")
            .status()
            .context("Couldn't run git (is it installed?)")?
    } else {
        Command::new("git")
            .arg("clone")
            .arg("--quiet")
            .arg("-n")
            .arg(&repo)
            .arg(path)
            .arg("--depth")
            .arg("1")
            .status()
            .context("Couldn't run git (is it installed?)")?
    };

    if verbose {
        println!("checking out on {:?}", path)
    }
    // Try to checkout the files
    let command_checkout = match clone_type {
        CloneType::Scheme => Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("checkout")
            .arg("--quiet")
            .arg("HEAD")
            .arg("*.y*ml")
            .status()
            .context("Couldn't run git (is it installed?)")?,
        CloneType::Template => Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("checkout")
            .arg("--quiet")
            .arg("HEAD")
            .arg("templates")
            .status()
            .context("Couldn't run git (is it installed?)")?,
        CloneType::List => Command::new("git")
            .arg("-C")
            .arg(path)
            .arg("checkout")
            .arg("--quiet")
            .arg("HEAD")
            .arg("list.yaml")
            .status()
            .context("Couldn't run git (is it installed?)")?,
    };

    let command_clone = command_clone
        .code()
        .ok_or_else(|| anyhow!("Failed to clone with git (is it installed?)"))?;
    let command_checkout = command_checkout
        .code()
        .ok_or_else(|| anyhow!("Failed to checkout files with git"))?;

    let _ = remove_dir_all(path.join(".git"));

    if command_clone != 0 || command_checkout != 0 {
        Err(anyhow!(
            "Git failed to run on repository '{}'. Check if your repo list is valid.",
            repo
        ))
    } else {
        Ok(())
    }
}

fn update_lists(
    dir: &Path,
    verbose: bool,
    config_path: &Path
) -> Result<()> {
    let sources_dir = &dir.join("sources");
    if verbose {
        println!("Updating sources list from sources.yaml")
    }

    // This section dealing with obtaining the config struct is copied directly from apply.rs
    //Check if config file exists
    if !config_path.exists() {
        eprintln!("Config {:?} doesn't exist, creating", config_path);
        let default_content = match read_to_string("/etc/flavours.conf") {
            Ok(content) => content,
            Err(_) => String::from(""),
        };
        let config_path_parent = config_path
            .parent()
            .with_context(|| format!("Couldn't get parent directory of {:?}", config_path))?;

        create_dir_all(config_path_parent).with_context(|| {
            format!(
                "Couldn't create configuration file parent directory {:?}",
                config_path_parent
            )
        })?;
        write(config_path, default_content)
            .with_context(|| format!("Couldn't create configuration file at {:?}", config_path))?;
    }

    let config_contents = read_to_string(config_path)
        .with_context(|| format!("Couldn't read configuration file {:?}.", config_path))?;

    let config = Config::read(&config_contents, config_path)?;

    // Get schemes and templates repository from file
    let (schemes_source, templates_source) = get_sources(
            &dir.join("sources.yaml"),
            &config 
    )?;
    if verbose {
        println!("Schemes source: {}", schemes_source);
        println!("Templates source: {}", templates_source);
    }

    // Spawn git clone threads, to clone schemes and templates lists
    let schemes_source_dir = sources_dir.join("schemes");
    let templates_source_dir = sources_dir.join("templates");
    let s_child = spawn(move || {
        git_clone(
            &schemes_source_dir,
            schemes_source,
            verbose,
            CloneType::List,
        )
    });
    let t_child = spawn(move || {
        git_clone(
            &templates_source_dir,
            templates_source,
            verbose,
            CloneType::List,
        )
    });

    // Execute and check exit code
    s_child.join().unwrap()?;
    t_child.join().unwrap()?;

    // write additional config sources
    let scheme_list = sources_dir.join("schemes").join("list.yaml");
    let template_list = sources_dir.join("templates").join("list.yaml");

    match config.extra_scheme {
        Some(extra_schemes) => {
            if let Ok(scheme_lines) = read_lines(&scheme_list) {
                // add new lines
                let mut lines: Vec<String> = scheme_lines.collect::<Result<_, _>>().unwrap();
                for es in &extra_schemes {
                    let text = format!("{}: {}", es.name, es.source);
                    lines.push(text);
                };

                // sort everything
                lines.sort();

                // save file
                let mut write_file = OpenOptions::new()
                    .write(true)
                    .open(&scheme_list)
                    .unwrap();
                for line in &lines {
                    if let Err(e) = writeln!(write_file, "{}", line) {
                        eprintln!("Couldn't write to file: {}", e);
                };
                };
            };
        },
        _ => ()
    };
    match config.extra_template {
        Some(extra_templates) => {
            if let Ok(template_lines) = read_lines(&template_list) {
                // add new lines
                let mut lines: Vec<String> = template_lines.collect::<Result<_, _>>().unwrap();
                for et in &extra_templates {
                    let text = format!("{}: {}", et.name, et.source);
                    lines.push(text);
                };

                // sort everything
                lines.sort();

                // save file
                let mut write_file = OpenOptions::new()
                    .write(true)
                    .open(&template_list)
                    .unwrap();
                for line in &lines {
                    if let Err(e) = writeln!(write_file, "{}", line) {
                        eprintln!("Couldn't write to file: {}", e);
                };
                }
            };
        },
        _ => ()
    };

    Ok(())
}

fn update_schemes(dir: &Path, verbose: bool) -> Result<()> {
    let schemes_dir = &dir.join("schemes");
    let scheme_list = &dir.join("sources").join("schemes").join("list.yaml");

    if verbose {
        println!("Updating schemes from source")
    }
    let schemes = get_repo_list(scheme_list)?;

    // Children for multithreaded processing
    let mut children = Vec::with_capacity(schemes.len());

    for (name, repo) in schemes {
        // Current scheme directory
        let current_dir = schemes_dir.join(name);
        // Spawn new thread
        children.push(spawn(move || {
            // Delete scheme directory and clone the repo
            git_clone(&current_dir, repo, verbose, CloneType::Scheme)
        }));
    }
    for child in children {
        // Unwrap thread result, then check git_clone return status with '?'
        child.join().unwrap()?;
    }
    // If no errors were raised, return as ok
    Ok(())
}

fn update_templates(dir: &Path, verbose: bool) -> Result<()> {
    let templates_dir = &dir.join("templates");
    let template_list = &dir.join("sources").join("templates").join("list.yaml");

    if verbose {
        println!("Updating templates from source")
    }
    let templates = get_repo_list(template_list)?;

    // Children for multithreaded processing
    let mut children = vec![];

    for template in templates {
        // Making copies of the variables to avoid problems with borrowing
        let (name, repo) = template;
        // Current template directory
        let current_dir = templates_dir.join(name);

        // Spawn new thread
        children.push(spawn(move || {
            // Delete template directory and clone the repo
            git_clone(&current_dir, repo, verbose, CloneType::Template)
        }));
    }
    for child in children {
        // Unwrap thread result, then check git_clone return status with '?'
        child.join().unwrap()?;
    }
    // If no errors were raised, return as ok
    Ok(())
}

///Implementation of update operation
///
///# Arguments
///* `operation` - Which operation to do
///* `dir` - The base path to be used
///* `verbose` - Boolean, be verbose if true
pub fn update(
    operation: &str,
    dir: &Path,
    verbose: bool,
    config_path: &Path
) -> Result<()> {
    let base16_dir = &dir.join("base16");
    create_dir_all(base16_dir)?;
    match operation {
        "lists" => update_lists(base16_dir, verbose, config_path),
        "schemes" => update_schemes(base16_dir, verbose),
        "templates" => update_templates(base16_dir, verbose),
        "all" => {
            update_lists(base16_dir, verbose, config_path)?;
            update_schemes(base16_dir, verbose)?;
            update_templates(base16_dir, verbose)
        }
        _ => Err(anyhow!("Invalid operation")),
    }
}
