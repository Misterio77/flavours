use std::path;
use std::fs;
use std::process;
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
    //Default repos
    let default_s_repo = "https://github.com/chriskempson/base16-schemes-source.git";
    let default_t_repo = "https://github.com/chriskempson/base16-templates-source.git";
    
    //Try to open file
    let sources_file = match fs::File::open(file) {
        //Success
        Ok(contents) => contents,
        //Handle error once, so if file is not found it can be created
        Err(_) => {
            //Try to write default repos to file
            write_sources(default_s_repo, default_t_repo, file)?;
            //Try to open it again, returns errors if unsucessful again
            fs::File::open(file)
                .with_context(||format!("Couldn't access {:?}", file))?
        },
    };
    //Variable to store repos, start with defaults (in case the file was read but didn't contain one or both repos
    let mut s_repo = String::from(default_s_repo);
    let mut t_repo = String::from(default_t_repo);

    //Bufreader from file
    let reader = io::BufReader::new(sources_file);
    //Iterate lines
    for line in reader.lines() {
        //Get name and repo from line
        let (name, repo) = parse_yml_line(line?)?;
        //Store in correct variable
        if name == "schemes" {
            s_repo = repo;
        } else if name == "templates" {
            t_repo = repo;
        }
    }

    //Rewrite file using found repository, this is done to clean up errors on the file or insert default values
    write_sources(&s_repo, &t_repo, file)?;

    //Return repos
    Ok((s_repo, t_repo))
}

fn get_repo_list(file: &path::Path) -> Result<Vec<(String, String)>> {
    let sources_file = fs::File::open(file)
        .with_context(|| format!("Failed to read repository list {:?}. Try 'update lists' first?", file))?;
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
fn git_clone(path: &path::Path, repo: &str, verbose: bool) -> Result<()> {
    //Remove directory, ignores errors
    let _ = fs::remove_dir_all(path);
    //Try to clone into directory
    let command;
    if verbose {
        command = process::Command::new("git")
                                       .arg("clone")
                                       .arg(repo)
                                       .arg(path)
                                       .status()?;
    } else {
        command = process::Command::new("git")
                                       .arg("clone")
                                       .arg("--quiet")
                                       .arg(repo)
                                       .arg(path)
                                       .status()?;
    }

    
    //Exit status code
    match command.code() {
        //If okay
        Some(0) => Ok(()),
        //If git returned an error
        Some(code) => Err(anyhow!("{}", code)),
        None => Err(anyhow!("Interrupted")),
    }
}


fn update_lists(dir: &path::Path, verbose:bool) -> Result<()> {
    let sources_dir = &dir.join("sources");
    if verbose { println!("Updating sources list from sources.yaml") }

    //Get schemes and templates repository from file
    let (schemes_source, templates_source) = get_sources(&dir.join("sources.yaml"))?; 
    if verbose {
        println!("Schemes source: {}", schemes_source);
        println!("Templates source: {}", templates_source);
    }

    //Clone schemes and templates repositories
    git_clone(&sources_dir.join("schemes"), &schemes_source, verbose)?;
    git_clone(&sources_dir.join("templates"), &templates_source, verbose)?;
    Ok(())
}

fn update_schemes(dir: &path::Path, verbose:bool) -> Result<()> {
    let schemes_dir = &dir.join("schemes");
    let scheme_list = &dir.join("sources").join("schemes").join("list.yaml");

    if verbose { println!("Updating schemes from source") }
    let schemes = get_repo_list(scheme_list)?;

    for scheme in schemes {
        let (name, repo) = scheme;
        if verbose{ println!("Downloading scheme {}", name) }
        git_clone(&schemes_dir.join(name), &repo, verbose)?;
    }
    Ok(())
}

fn update_templates(dir: &path::Path, verbose:bool) -> Result<()> {
    let templates_dir = &dir.join("templates");
    let template_list = &dir.join("sources").join("templates").join("list.yaml");

    if verbose { println!("Updating templates from source") }
    let templates = get_repo_list(template_list)?;

    for template in templates {
        let (name, repo) = template;
        if verbose{ println!("Downloading template {}", name) }
        git_clone(&templates_dir.join(name), &repo, verbose)?;
    }
    Ok(())
}

///Implementation of update operation
///
///# Arguments
///* `arguments` - A clap argmatches instance, for the update subcommand
///* `dir` - The base16 path to be used
///* `verbose` - Boolean, be verbose if true
pub fn update(arguments: &clap::ArgMatches, dir: &path::Path, verbose: bool) -> Result<()> {
    let base16_dir = &dir.join("base16");
    match arguments.value_of("operation") {
        Some("lists") => update_lists(base16_dir, verbose),
        Some("schemes") => update_schemes(base16_dir, verbose),
        Some("templates") => update_templates(base16_dir, verbose),
        Some("all") => {
            update_lists(base16_dir, verbose)?;
            update_schemes(base16_dir, verbose)?;
            update_templates(base16_dir, verbose)
        }
        Some(_) | None => Err(anyhow!("Invalid operation"))
    }
}

