use std::path::Path;
use std::fs::{remove_dir_all, read_to_string, write};
use std::process::Command;
use std::io::Error;
use std::result::Result;

extern crate dirs;
use dirs::data_dir;

extern crate clap;
use clap::ArgMatches;

static DEFAULT_SCHEMES_SOURCE: &str = "https://github.com/chriskempson/base16-schemes-source.git";
static DEFAULT_TEMPLATES_SOURCE: &str = "https://github.com/chriskempson/base16-templates-source.git";

fn write_default_sources(dir: &Path) -> Result<(), Error> {
    let file = &dir.join("sources.yaml");
    let text = format!("schemes: {}\ntemplates: {}", DEFAULT_SCHEMES_SOURCE, DEFAULT_TEMPLATES_SOURCE);

    write(file, text)?;
    return Ok(());
}

fn git_clone(path: &Path, repo: &str) -> bool {
    let _ = remove_dir_all(path);
    return Command::new("git")
                   .arg("clone")
                   .arg("--quiet")
                   .arg(repo)
                   .arg(path)
                   .status()
                   .expect("Error cloning repository")
                   .success()
}

pub fn update(arguments: &ArgMatches) {
    extern crate dirs;
    
    let dir = data_dir().unwrap().join("flavours").join("base16");

    //Check which update operation was used
    //(We can safely unwrap, clap handles errors or missing arguments)
    let operation = arguments.value_of("operation").unwrap();

    if operation == "lists"    || operation == "all" {
        let sources_file_contents =  read_to_string(&dir.join("sources.yaml")); 
        let sources_dir = &dir.join("sources");
        //git_clone(&sources_dir.join("schemes"), schemes_source);
        //git_clone(&sources_dir.join("templates"), templates_source);
    }
    if operation == "schemes"  || operation == "all" {
        println!("schemes");
    }
    if operation == "templates"|| operation == "all" {
        println!("templates");
    }
}

