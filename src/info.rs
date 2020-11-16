use std::fs;
use std::path;

use anyhow::{anyhow, Context, Result};

#[path = "find.rs"]
mod find;

#[path = "scheme.rs"]
mod scheme;

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
pub fn info(patterns: Vec<&str>, base_dir: &path::Path, color: bool) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find::find(pattern, &base_dir.join("base16").join("schemes"))?;

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

        let scheme = scheme::parse_scheme(
            &fs::read_to_string(&scheme_file)
                .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?,
            scheme_slug,
        )?;

        println!("{} ({})", scheme.scheme, scheme_slug);
        println!("by {}", scheme.author);
        if !color {
            println!("#{}", scheme.base00);
            println!("#{}", scheme.base01);
            println!("#{}", scheme.base02);
            println!("#{}", scheme.base03);
            println!("#{}", scheme.base04);
            println!("#{}", scheme.base05);
            println!("#{}", scheme.base06);
            println!("#{}", scheme.base07);
            println!("#{}", scheme.base08);
            println!("#{}", scheme.base09);
            println!("#{}", scheme.base0A);
            println!("#{}", scheme.base0B);
            println!("#{}", scheme.base0C);
            println!("#{}", scheme.base0D);
            println!("#{}", scheme.base0E);
            println!("#{}", scheme.base0F);
        } else {
            print_color(&scheme.base00)?;
            print_color(&scheme.base01)?;
            print_color(&scheme.base02)?;
            print_color(&scheme.base03)?;
            print_color(&scheme.base04)?;
            print_color(&scheme.base05)?;
            print_color(&scheme.base06)?;
            print_color(&scheme.base07)?;
            print_color(&scheme.base08)?;
            print_color(&scheme.base09)?;
            print_color(&scheme.base0A)?;
            print_color(&scheme.base0B)?;
            print_color(&scheme.base0C)?;
            print_color(&scheme.base0D)?;
            print_color(&scheme.base0E)?;
            print_color(&scheme.base0F)?;
        }
    }

    Ok(())
}
