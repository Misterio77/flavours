use std::env::set_var;
use std::fs::{self, create_dir_all, remove_dir_all, write, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use std::thread::spawn;

use anyhow::{anyhow, Context, Result};

use crate::config::Config;

///Parses yml line containing name and repository link
///
///# Arguments
///* `line` - String containing both name and repository in format `name: repository`
fn parse_yml_line(line: String) -> Result<(String, String)> {
    let parsed: Vec<&str> = line.split(':').collect();
    let name = String::from(parsed[0]);
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
    let text = format!("schemes: {}\ntemplates: {}", s_repo, t_repo);
    write(file, text).with_context(|| format!("Couldn't write {:?}", file))?;
    Ok(())
}

///Gets schemes and repository links from file, falls back to default repos
///
///# Arguments
///* `file` - Path to sources file
fn get_sources(config_dir: &Path, file: &Path) -> Result<(String, String)> {
    let config_path = &config_dir.join("config.toml");
    let config_contents = fs::read_to_string(config_path)
        .with_context(|| format!("Couldn't read configuration file {:?}.", config_path))?;
    let config = Config::read(&config_contents, config_path)?;

    // Default repos
    let default_s_repo = "https://github.com/chriskempson/base16-schemes-source.git";
    let default_t_repo = "https://github.com/chriskempson/base16-templates-source.git";

    let mut s_repo = config.schemes_url.unwrap_or(String::from(default_s_repo));
    let mut t_repo = config.templates_url.unwrap_or(String::from(default_t_repo));

    if s_repo.eq(default_s_repo) || t_repo.eq(default_t_repo) {
        let config_path = config_dir.join("sources.yaml");
        let sources_file;

        if config_path.exists() {
            sources_file = File::open(config_path)?;
            (s_repo, t_repo) = extract_repos(sources_file, s_repo, t_repo)?;
        } else if file.exists() {
            sources_file = File::open(file)?;
            (s_repo, t_repo) = extract_repos(sources_file, s_repo, t_repo)?;
        }
    }

    write_sources(&s_repo, &t_repo, file)?;
    Ok((s_repo, t_repo))
}

fn extract_repos(
    sources_file: File,
    mut s_repo: String,
    mut t_repo: String,
) -> Result<(String, String)> {
    // Bufreader from file
    let reader = BufReader::new(sources_file);
    // Iterate lines
    for line in reader.lines() {
        // Get name and repo from line
        let (name, repo) = parse_yml_line(line?)?;
        // Store in correct variable
        if name == "schemes" {
            s_repo = repo;
        } else if name == "templates" {
            t_repo = repo;
        }
    }
    return Ok((s_repo, t_repo));
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
        let (name, repo) = parse_yml_line(line?)?;
        let first = name.chars().next();
        if first != Some('#') && first != None {
            result.push((name, repo));
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

fn update_lists(config_dir: &Path, base_dir: &Path, verbose: bool) -> Result<()> {
    let sources_dir = &base_dir.join("sources");
    if verbose {
        println!("Updating sources list from sources.yaml")
    }

    // Get schemes and templates repository from file
    let (schemes_source, templates_source) =
        get_sources(&config_dir, &base_dir.join("sources.yaml"))?;
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
    let mut children = vec![];

    for scheme in schemes {
        // Making copies of the variables to avoid problems with borrowing
        let (name, repo) = scheme;
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
///* `config_path` - The config path
///* `verbose` - Boolean, be verbose if true
pub fn update(operation: &str, dir: &Path, config_dir: &Path, verbose: bool) -> Result<()> {
    let base16_dir = &dir.join("base16");
    create_dir_all(base16_dir)?;
    match operation {
        "lists" => update_lists(config_dir, base16_dir, verbose),
        "schemes" => update_schemes(base16_dir, verbose),
        "templates" => update_templates(base16_dir, verbose),
        "all" => {
            update_lists(config_dir, base16_dir, verbose)?;
            update_schemes(base16_dir, verbose)?;
            update_templates(base16_dir, verbose)
        }
        _ => Err(anyhow!("Invalid operation")),
    }
}
