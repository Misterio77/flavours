use std::path;
use std::fs;
use std::process;
use std::io::{self, BufRead};

extern crate anyhow;
use anyhow::{anyhow, Result};

extern crate dirs;
extern crate clap;

///Writes schemes and templates repos information 
///
///# Arguments
///* `s_repo` - String slice containing Schemes repository link
///* `t_repo` - String slice containing Schemes repository link
///* `file` - Path to sources file
fn write_repos(s_repo: &str, t_repo: &str, file: &path::Path) -> Result<()> {
    let text = format!("schemes: {}\ntemplates: {}", s_repo, t_repo);
    fs::write(file, text)?;
    Ok(())
}

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

///Gets schemes and repository links from file, falls back to default repos
///
///# Arguments
///* `file` - Path to sources file
fn  get_repos(file: &path::Path) -> Result<(String, String)> {
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
            write_repos(default_s_repo, default_t_repo, file)?;
            //Try to open it again, returns errors if unsucessful again
            fs::File::open(file)?
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
    write_repos(&s_repo, &t_repo, file)?;

    //Return repos
    Ok((s_repo, t_repo))
}

///Uses git to clone a repository
///
///# Arguments
///* `path` - File path where repository should be cloned
///* `repo` - String slice containing link to repository
fn git_clone(path: &path::Path, repo: &str) -> Result<()> {
    //Remove directory, ignores errors
    let _ = fs::remove_dir_all(path);
    //Try to clone into directory
    let status = process::Command::new("git")
                                  .arg("clone")
                                  .arg("--quiet")
                                  .arg(repo)
                                  .arg(path)
                                  .status()?;

    //Exit status code
    match status.code() {
        //If okay
        Some(0) => Ok(()),
        //If git returned an error
        Some(code) => Err(anyhow!("{}", code)),
        None => Err(anyhow!("Interrupted")),
    }
}


///Implementation of update operation
///
///# Arguments
///* `arguments` - A clap argmatches instance, for the update subcommand
pub fn update(arguments: &clap::ArgMatches) -> Result<()> {
    extern crate dirs;
    
    //Unwrap user data directory, join flavours and base16 directories to path
    let dir = dirs::data_dir().unwrap().join("flavours").join("base16");

    //Check which update operation was used
    //(We can safely unwrap, clap handles errors or missing arguments)
    let operation = arguments.value_of("operation").unwrap();

    //Operation update lists
    if operation == "lists"    || operation == "all" {
        //Get schemes and templates repository from file
        let (schemes_source, templates_source) = get_repos(&dir.join("sources.yaml"))?; 
        //Sources directory
        let sources_dir = &dir.join("sources");

        //Clone schemes and templates repositories
        git_clone(&sources_dir.join("schemes"), &schemes_source)?;
        git_clone(&sources_dir.join("templates"), &templates_source)?;
    }
    //Operation update schemes
    if operation == "schemes"  || operation == "all" {
        println!("schemes");
    }
    //Operation update templates
    if operation == "templates" || operation == "all" {
        println!("templates");
    }
    //When done
    return Ok(());
}

