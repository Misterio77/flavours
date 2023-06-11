use anyhow::{anyhow, Context, Result};
use base16_color_scheme::Scheme;
use std::fs::read_to_string;
use std::path::{self, Path, PathBuf};
use std::{fmt, fs};

use crate::find::find_schemes;

/// Get the name of the current scheme
///
/// * `dir` - flavours data directory
fn get_current_scheme_name(dir: &Path) -> Result<String> {
    // File that stores last used scheme
    let file_path = &dir.join("lastscheme");
    // Try to open it
    let scheme: String = read_to_string(file_path)
        .with_context(|| "Failed to read last scheme file. Try applying first.")?
        .split_whitespace()
        .collect();

    if scheme.is_empty() {
        Err(anyhow!(
            "Failed to read last scheme from file. Try applying first."
        ))
    } else {
        Ok(scheme)
    }
}

fn get_current_scheme(scheme_name: String, base_dir: &Path, config_dir: &Path) -> Result<Scheme> {
    let schemes = find_schemes(&scheme_name, base_dir, config_dir)?;
    let scheme_file: &PathBuf = schemes
        .first()
        .with_context(|| "Could not find any schemes")?;

    //Read chosen scheme
    let scheme_contents = fs::read_to_string(&scheme_file)
        .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;
    let scheme = serde_yaml::from_str(&scheme_contents)?;
    Ok(scheme)
}

/// Luminosity of a theme
#[derive(Debug)]
enum Luminosity {
    Dark,
    Light,
}

impl fmt::Display for Luminosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Luminosity::Dark => write!(f, "Dark"),
            Luminosity::Light => write!(f, "Light"),
        }
    }
}

/// Get the luminosity of the current theme
///
/// * ``
fn get_luminosity(scheme: Scheme) -> Result<Luminosity> {
    // FOR TESTING PURPOSES, JUST RETURN DARK.
    println!("{:?}", scheme);
    return Ok(Luminosity::Dark);
}

/// Current subcommand
///
/// * `base_dir` - flavours data directory
/// * `luminosity` - Whether or not to return the luminosity of the current theme
/// * `verbose` - Should we be verbose (unused atm)
pub fn current(base_dir: &Path, config_dir: &Path, luminosity: bool, _verbose: bool) -> Result<()> {
    if luminosity {
        let scheme_luminosity = get_luminosity(get_current_scheme(
            get_current_scheme_name(base_dir)?,
            base_dir,
            config_dir,
        )?)?;
        println!("{}", scheme_luminosity)
    } else {
        println!("{}", get_current_scheme_name(base_dir)?);
    }
    Ok(())
}
