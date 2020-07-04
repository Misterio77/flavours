use std::path;

use anyhow::{Result, anyhow};

#[path = "find.rs"]
mod find;

pub fn list(arguments: &clap::ArgMatches, base_dir: &path::Path, _verbose: bool) -> Result<()> {
    let patterns = match arguments.values_of("pattern") {
        Some(values) => values.collect(),
        //If none is supplied, defaults to wildcard
        None => vec!["*"],
    };

    let mut schemes = Vec::new();
    for pattern in patterns {        
        let found_schemes = find::find(pattern, &base_dir.join("base16").join("schemes"))?;
        for found_scheme in found_schemes {
            schemes.push(
                String::from(
                    found_scheme
                    .file_stem().ok_or(anyhow!("Couldn't get scheme name"))?
                    .to_str().ok_or(anyhow!("Couldn't convert name"))?
                )
            );
        }
    }


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
