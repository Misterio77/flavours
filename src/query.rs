use std::path;
use std::fs;
use std::env;
use glob::glob;

use anyhow::{Result, anyhow, Context};

fn last_scheme(dir: &path::Path) -> Result<String> {
    //File that stores last used scheme
    let file_path = &dir.join("lastscheme");
    //Try to open it
    let scheme = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read last scheme from {:?}. Try applying one first", file_path))?;

    if scheme == "" {
        Err(anyhow!("Last scheme file is empty. Try applying one first"))
    } else {
        Ok(scheme)
    }
}

fn find_file(pattern: String, base_dir: &path::Path, mut matches: Vec<String>) -> Result<Vec<String>> {
    //Store old working directory
    let old_working_dir = env::current_dir()?;
    //Change to the schemes directory
    env::set_current_dir(&base_dir.join("base16").join("schemes"))?;
    
    //Use glob to search with pattern, then iterate
    for entry in glob(&pattern).unwrap() {
        //For every found file, we'll get only the stem (name without extension), turn into a string, and add to the supplied vector
        matches.push( match entry?.file_stem() {
            Some(value) => match value.to_str() {
                Some (contents) => String::from(contents),
                None => return Err(anyhow!("Error converting string")),
            }
            None => return Err(anyhow!("Error converting scheme path to string")),
        })
    }
    //Change back to old working directory
    env::set_current_dir(old_working_dir)?;
    //Return matches
    Ok(matches)
}


pub fn find_schemes(pattern_in: Option<clap::Values>, base_dir: &path::Path) -> Result<Vec<String>> {
    //String Vec that stores all given arguments
    let input = match pattern_in {
        Some(values) => {
            //Create a vector
            let mut vec = Vec::new();
            //Populate it with every element turned into String
            for value in values {
                vec.push(String::from(value));
            }
            //Return it
            vec
        },
        //If none is supplied, get last applied scheme
        None => vec!(last_scheme(base_dir)?),
    };

    //String vec that will contain all matches
    let mut matches = Vec::new();

    //Iterate input vector
    for element in input {
        //Search twice, once for .yaml and another for .yml schemes
        //Find_file will get the matches and add them to the vector
        let pattern = format!("*/{}.yaml", element);
        matches = find_file(pattern, base_dir, matches)?;

        let pattern = format!("*/{}.yml", element);
        matches = find_file(pattern, base_dir, matches)?;
    }
    //Sort vector
    matches.sort();
    //Remove duplicates
    matches.dedup();

    //Return it
    Ok(matches)

}

pub fn query(arguments: &clap::ArgMatches, base_dir: &path::Path, _verbose: bool) -> Result<()> {
    //TODO: find info worth printing for verbose option
    //Get schemes
    let schemes = find_schemes(arguments.values_of("pattern"), base_dir)?;
    //Print them
    for scheme in schemes {
        println!("{}", scheme);
    }

    Ok(())
}
