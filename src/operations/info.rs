use anyhow::{anyhow, Context, Result};
use std::fs::read_to_string;
use std::path::Path;

use crate::find::find;
use crate::scheme::Scheme;

fn true_color(hex_color: &str, background: bool) -> Result<String> {
    let rgb = hex::decode(hex_color)?;

    let code = if background { 48 } else { 38 };

    Ok(format!("\x1b[{};2;{};{};{}m", code, rgb[0], rgb[1], rgb[2]))
}

pub fn print_color(color: &str) -> Result<()> {
    const RESETCOLOR: &str = "\x1b[0m";
    println!(
        "{} #{} {}  {}#{}{}",
        true_color(&color, true)?,
        color,
        RESETCOLOR,
        true_color(&color, false)?,
        color,
        RESETCOLOR
    );
    Ok(())
}

/// Info subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours base data dir
/// * `verbose` - Should we be verbose? (unused)
/// * `color` - Should we print with colors?
pub fn info(patterns: Vec<&str>, base_dir: &Path, raw: bool) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find(pattern, &base_dir.join("base16").join("schemes"))?;

        for found_scheme in found_schemes {
            schemes.push(found_scheme);
        }
    }
    schemes.sort();
    schemes.dedup();

    if schemes.is_empty() {
        return Err(anyhow!("No matching scheme found"));
    };

    let mut first = true;
    for scheme_file in schemes {
        if first {
            first = false;
        } else {
            println!();
        }
        let scheme_slug = scheme_file
            .file_stem()
            .ok_or_else(|| anyhow!("Couldn't get scheme name."))?
            .to_str()
            .ok_or_else(|| anyhow!("Couldn't convert scheme file name."))?;
        let scheme_contents = read_to_string(&scheme_file)
            .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;

        let scheme = Scheme::from_str(&scheme_contents, scheme_slug)?;

        println!("{} ({})", scheme.name, scheme.slug);
        println!("by {}", scheme.author);
        if raw {
            for color in scheme.colors.iter() {
                println!("#{}", color);
            }
        } else {
            for color in scheme.colors.iter() {
                print_color(color)?;
            }
        }
    }

    Ok(())
}
