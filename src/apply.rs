use std::path;

use anyhow::{Result, anyhow};
use rand::seq::SliceRandom;

fn random(values: Vec<&str>) -> Result<&str> {
    match values.choose(&mut rand::thread_rng()) {
        Some(chosen) => Ok(chosen),
        None => Err(anyhow!("Error getting random scheme")),
    }
}

pub fn apply(arguments: &clap::ArgMatches, base_dir: &path::Path, verbose: bool) -> Result<()> {
    //Get all supplied schemes, turn into vector of strings
    let supplied_schemes = match arguments.values_of("scheme") {
        Some(schemes) => schemes.collect(),
        None => return Err(anyhow!("You need to supply at least one scheme")),
    };
    //Call random function
    let scheme = random(supplied_schemes)?;

    println!("{}", scheme);
    Ok(())
}
