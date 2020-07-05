use std::path;

use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;

#[path = "find.rs"]
mod find;

fn random(values: Vec<path::PathBuf>) -> Result<path::PathBuf> {
    let chosen = values
        .choose(&mut rand::thread_rng())
        .ok_or(
            anyhow!("Error getting random value")
        )?;
    Ok(chosen.to_path_buf())
}

pub fn apply(arguments: &clap::ArgMatches, base_dir: &path::Path, verbose: bool) -> Result<()> {
    let patterns = match arguments.values_of("pattern") {
        Some(schemes) => schemes.collect(),
        //If none is supplied, defaults to wildcard
        None => vec!["*"],
    };

    //Build vec with all matching schemes
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find::find(
            pattern,
            &base_dir.join("base16").join("schemes")
        )?;

        for found_scheme in found_schemes {
            schemes.push(found_scheme);
        }
    }
    schemes.sort();
    schemes.dedup();

    //Call random function
    let chosen = random(schemes)?;

    if verbose { println!("Chosen scheme: {:?}", chosen) };

    Ok(())
}
