use std::path;
use std::env;
use glob::glob;

use anyhow::{Result, anyhow};

fn find_files(pattern: String, dir: &path::Path) -> Result<Vec<String>> {
    //Matches vector to be returned
    let mut matches = Vec::new();

    //Store old working directory
    let old_working_dir = env::current_dir()?;
    //Change to the schemes directory
    env::set_current_dir(dir)?;
    
    //Use glob to search with pattern, then iterate
    for entry in glob(&pattern).unwrap() {
        //For every found file, we'll get only the stem (name without extension), turn into a string, and add to the vector
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
    //String Vec that stores all given patterns
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
        //If none is supplied, defaults to wildcard
        None => vec!(String::from("*")),
    };

    //String vec that will contain all matching schemes
    let mut matches = Vec::new();

    //Iterate input vector
    for element in input {
        //Search twice, once for .yaml and another for .yml schemes
        //Find_file will get the matches and add them to the vector
        let schemes_dir = base_dir.join("base16").join("schemes");
        let pattern = format!("*/{}.yaml", element);
        matches.append(&mut find_files(pattern, &schemes_dir)?);

        let pattern = format!("*/{}.yml", element);
        matches.append(&mut find_files(pattern, &schemes_dir)?);
    }
    //Sort vector
    matches.sort();
    //Remove duplicates
    matches.dedup();

    //Return it
    Ok(matches)

}

pub fn list(arguments: &clap::ArgMatches, base_dir: &path::Path, _verbose: bool) -> Result<()> {
    //TODO: find info worth printing for verbose option
    //Get schemes
    let schemes = find_schemes(arguments.values_of("pattern"), base_dir)?;

    //Should we print a new line for each scheme?
    let lines = arguments.is_present("lines");


    for scheme in schemes {
        //Print scheme
        print!("{}", scheme);
        if lines {
            //Print newline
            println!("");
        } else {
            //Print space
            print!(" ");
        }
    }

    //If we separated by spaces, print an ending newline
    if !lines { println!(""); }

    Ok(())
}
