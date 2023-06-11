use anyhow::{anyhow, Context, Result};
use base16_color_scheme::scheme::{BaseIndex, RgbColor};
use base16_color_scheme::Scheme;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::{fmt, fs, vec};

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
enum Luminance {
    Dark,
    Light,
}

impl fmt::Display for Luminance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Luminance::Dark => write!(f, "Dark"),
            Luminance::Light => write!(f, "Light"),
        }
    }
}

/// Get the luminance of the current theme
///
/// * ``
fn get_luminance(scheme: Scheme) -> Luminance {
    let rgb2luminance = |rgb: &RgbColor| {
        let [r, g, b] = rgb.0;

        // there are exacter ways, this turns out to be good enough
        // https://www.w3.org/TR/AERT/#color-contrast
        let luminance = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
        luminance / 255.0
    };
    // Take into account the main background colors as per the styling guide
    // https://github.com/tinted-theming/home/blob/main/styling.md
    let background_indices = vec![0, 1];

    let luminances: Vec<_> = background_indices
        .into_iter()
        .map(|idx| rgb2luminance(scheme.colors.get(&BaseIndex(idx)).unwrap()))
        .collect();

    let avg_luminance: f32 = luminances.iter().sum::<f32>() / luminances.len() as f32;
    if avg_luminance < 0.5 {
        Luminance::Dark
    } else {
        Luminance::Light
    }
}

/// Current subcommand
///
/// * `base_dir` - flavours data directory
/// * `luminosity` - Whether or not to return the luminosity of the current theme
/// * `verbose` - Should we be verbose (unused atm)
pub fn current(base_dir: &Path, config_dir: &Path, luminosity: bool, _verbose: bool) -> Result<()> {
    if luminosity {
        let scheme_luminance = get_luminance(get_current_scheme(
            get_current_scheme_name(base_dir)?,
            base_dir,
            config_dir,
        )?);
        println!("{}", scheme_luminance)
    } else {
        println!("{}", get_current_scheme_name(base_dir)?);
    }
    Ok(())
}
