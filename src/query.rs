use std::path;
use std::fs;
use std::env;
use glob::glob;

use anyhow::{Result, anyhow};

fn last_scheme(dir: &path::Path) -> Result<String> {
    let file_path = &dir.join("lastscheme");
    let scheme = match fs::read_to_string(file_path) {
        Ok(value) => value.split_whitespace().collect(),
        Err(_) => String::from(""),
    };
    return Ok(scheme);
}

fn find_file(pattern: String, dir: &path::Path, mut matches: Vec<String>) -> Result<Vec<String>> {
    let old_working_dir = env::current_dir()?;
    env::set_current_dir(&dir.join("base16").join("schemes"))?;
    
    for entry in glob(&pattern).unwrap() {
        matches.push( match entry?.file_stem() {
            Some(value) => match value.to_str() {
                Some (contents) => String::from(contents),
                None => return Err(anyhow!("Error converting string")),
            }
            None => return Err(anyhow!("Error converting scheme path to string")),
        })
    }
    env::set_current_dir(old_working_dir)?;
    Ok(matches)
}


pub fn find_scheme(pattern_in: Option<clap::Values>, dir: &path::Path) -> Result<Vec<String>> {
    let input = match pattern_in {
        Some(values) => {
            let mut vec = Vec::new();
            for value in values {
                vec.push(String::from(value));
            }
            vec
        },
        None => vec!(last_scheme(dir)?),
    };

    let mut matches = Vec::new();

    for element in input {
        let pattern = format!("*/{}.yaml", element);
        matches = find_file(pattern, dir, matches)?;
        let pattern = format!("*/{}.yml", element);
        matches = find_file(pattern, dir, matches)?;
    }
    matches.sort();
    matches.dedup();

    Ok(matches)

}

pub fn query(arguments: &clap::ArgMatches, dir: &path::Path, _verbose: bool) -> Result<()> {
    let schemes = find_scheme(arguments.values_of("query-pattern"), dir)?;
    for scheme in schemes {
        println!("{}", scheme);
    }

    Ok(())
}
