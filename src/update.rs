use std::path;
use std::fs;
use std::process;
use std::thread;
use std::env;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result, Context};

///Parses yml line containing name and repository link
///
///# Arguments
///* `line` - String containing both name and repository in format `name: repository`
fn parse_yml_line(line: String) -> Result<(String, String)> {
    let parsed: Vec<&str> = line.split(":").collect();
    let name = String::from(parsed[0]);
    let repo = parsed[1..].join(":").split_whitespace().collect();
    return Ok((name, repo));
}

///Writes schemes and templates repos information 
///
///# Arguments
///* `s_repo` - String slice containing Schemes repository link
///* `t_repo` - String slice containing Schemes repository link
///* `file` - Path to sources file
fn write_sources(s_repo: &str, t_repo: &str, file: &path::Path) -> Result<()> {
    let text = format!("schemes: {}\ntemplates: {}", s_repo, t_repo);
    fs::write(file, text)
        .with_context(|| format!("Couldn't write {:?}", file))?;
    Ok(())
}

///Gets schemes and repository links from file, falls back to default repos
///
///# Arguments
///* `file` - Path to sources file
fn get_sources(file: &path::Path) -> Result<(String, String)> {
    // Default repos
    let default_s_repo = "https://github.com/chriskempson/base16-schemes-source.git";
    let default_t_repo = "https://github.com/chriskempson/base16-templates-source.git";
    
    // Try to open file
    let sources_file = match fs::File::open(file) {
        // Success
        Ok(contents) => contents,
        // Handle error once, so if file is not found it can be created
        Err(_) => {
            // Try to write default repos to file
            write_sources(default_s_repo, default_t_repo, file)?;
            // Try to open it again, returns errors if unsucessful again
            fs::File::open(file)
                .with_context(||format!("Couldn't access {:?}", file))?
        }
    };
    // Variable to store repos, start with defaults (in case the file was read but didn't contain one or both repos
    let mut s_repo = String::from(default_s_repo);
    let mut t_repo = String::from(default_t_repo);

    // Bufreader from file
    let reader = io::BufReader::new(sources_file);
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

    // Rewrite file using found repository, this is done to clean up errors on the file or insert default values
    write_sources(&s_repo, &t_repo, file)?;

    // Return repos
    Ok((s_repo, t_repo))
}
///Get name and repository vector from given list
///
///# Arguments
///* `file` - File with list
fn get_repo_list(file: &path::Path) -> Result<Vec<(String, String)>> {
    let sources_file = fs::File::open(file)
        .with_context(||
            format!("Failed to read repository list {:?}. Try 'update lists' first?", file)
        )?;
    let mut result = Vec::new();

    let reader = io::BufReader::new(sources_file);

    for line in reader.lines() {
        let (name, repo) = parse_yml_line(line?)?;
        let first = name.chars().next();
        if first != Some('#') && first != None {
            result.push((name, repo));
        }
    }

    return Ok(result);

}

///Uses git to clone a repository
///
///# Arguments
///* `path` - File path where repository should be cloned
///* `repo` - String slice containing link to repository
///* `verbose` - Boolean, tell git to be quiet if false
fn git_clone(path: &path::Path, repo: String, verbose: bool) -> Result<()> {
    // Remove directory, ignores errors
    let _ = fs::remove_dir_all(path);
    env::set_var("GIT_TERMINAL_PROMPT", "0");

    // Try to clone into directory
    let command;
    if verbose {
        command = process::Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg(&repo)
            .arg(path)
            .status()
            .with_context(||
                format!("Failed to run git, is it installed?")
            )?;
    } else {
        command = process::Command::new("git")
            .arg("clone")
            .arg("--depth")
            .arg("1")
            .arg("--quiet")
            .arg(&repo)
            .arg(path)
            .status()
            .with_context(||
                format!("Failed to run git, is it installed?")
            )?;
    }

    
    // Exit status code
    match command.code() {
        // If okay
        Some(0) => Ok(()),
        // If git returned an error
        Some(_) | None => {
            Err(
                anyhow!("Git clone failed on repo '{}', check if your repository lists are correct.", &repo)
            )
        }
    }
}


fn update_lists(dir: &path::Path, verbose:bool) -> Result<()> {
    let sources_dir = &dir.join("sources");
    if verbose { println!("Updating sources list from sources.yaml") }

    // Get schemes and templates repository from file
    let (schemes_source, templates_source) = get_sources(&dir.join("sources.yaml"))?; 
    if verbose {
        println!("Schemes source: {}", schemes_source);
        println!("Templates source: {}", templates_source);
    }

    // Spawn git clone threads, to clone schemes and templates lists
    let schemes_source_dir = sources_dir.join("schemes");
    let templates_source_dir = sources_dir.join("templates");
    let s_child = thread::spawn(move || {
        git_clone(&schemes_source_dir, schemes_source, verbose)
    });
    let t_child = thread::spawn(move || {
        git_clone(&templates_source_dir, templates_source, verbose)
    });

    // Execute and check exit code
    s_child.join().unwrap()?;
    t_child.join().unwrap()?;
    
    Ok(())
}

fn update_schemes(dir: &path::Path, verbose:bool) -> Result<()> {
    let schemes_dir = &dir.join("schemes");
    let scheme_list = &dir.join("sources").join("schemes").join("list.yaml");

    if verbose { println!("Updating schemes from source") }
    let schemes = get_repo_list(scheme_list)?;

    // Children for multithreaded processing
    let mut children = vec![];

    for scheme in schemes {
        // Making copies of the variables to avoid problems with borrowing
        let (name, repo) = scheme;
        // Current scheme directory
        let current_dir = schemes_dir.join(name);
        // Spawn new thread
        children.push(thread::spawn(move || {
            // Delete scheme directory and clone the repo
            git_clone(&current_dir, repo, verbose) 
        }));
    }
    for child in children {
        // Unwrap thread result, then check git_clone return status with '?'
        child.join().unwrap()?;
    }
    // If no errors were raised, return as ok
    Ok(())
}

fn update_templates(dir: &path::Path, verbose:bool) -> Result<()> {
    let templates_dir = &dir.join("templates");
    let template_list = &dir.join("sources").join("templates").join("list.yaml");

    if verbose { println!("Updating templates from source") }
    let templates = get_repo_list(template_list)?;

    // Children for multithreaded processing
    let mut children = vec![];

    for template in templates {
        // Making copies of the variables to avoid problems with borrowing
        let (name, repo) = template;
        // Current template directory
        let current_dir = templates_dir.join(name);

        // Spawn new thread
        children.push(thread::spawn(move || {
            // Delete template directory and clone the repo
            git_clone(&current_dir, repo, verbose) 
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
///* `arguments` - A clap argmatches instance, for the update subcommand
///* `dir` - The base path to be used
///* `verbose` - Boolean, be verbose if true
pub fn update(operation: &str, dir: &path::Path, verbose: bool) -> Result<()> {
    let base16_dir = &dir.join("base16");
    fs::create_dir_all(base16_dir)?;
    match operation {
        "lists" => update_lists(base16_dir, verbose),
        "schemes" => update_schemes(base16_dir, verbose),
        "templates" => update_templates(base16_dir, verbose),
        "all" => {
            update_lists(base16_dir, verbose)?;
            update_schemes(base16_dir, verbose)?;
            update_templates(base16_dir, verbose)
        }
        _ => Err(anyhow!("Invalid operation"))
    }
}

