use std::path;
use std::fs;

use anyhow::{anyhow, Result, Context};

#[path = "find.rs"]
mod find;

#[path = "scheme.rs"]
mod scheme;

const RESETCOLOR: &str = "\x1b[0m";

fn true_color(hex_color: &str, background: bool) -> Result<String> {
    let rgb = hex::decode(hex_color)?;

    let code = if background {
        48
    } else {
        38
    }; 

    Ok(format!("\x1b[{};2;{};{};{}m", code, rgb[0], rgb[1], rgb[2]))
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
        return Err(anyhow!("No matching scheme found"))
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
            scheme_slug
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
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base00, true)?, scheme.base00, RESETCOLOR, true_color(&scheme.base00, false)?, scheme.base00, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base01, true)?, scheme.base01, RESETCOLOR, true_color(&scheme.base01, false)?, scheme.base01, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base02, true)?, scheme.base02, RESETCOLOR, true_color(&scheme.base02, false)?, scheme.base02, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base03, true)?, scheme.base03, RESETCOLOR, true_color(&scheme.base03, false)?, scheme.base03, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base04, true)?, scheme.base04, RESETCOLOR, true_color(&scheme.base04, false)?, scheme.base04, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base05, true)?, scheme.base05, RESETCOLOR, true_color(&scheme.base05, false)?, scheme.base05, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base06, true)?, scheme.base06, RESETCOLOR, true_color(&scheme.base06, false)?, scheme.base06, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base07, true)?, scheme.base07, RESETCOLOR, true_color(&scheme.base07, false)?, scheme.base07, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base08, true)?, scheme.base08, RESETCOLOR, true_color(&scheme.base08, false)?, scheme.base08, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base09, true)?, scheme.base09, RESETCOLOR, true_color(&scheme.base09, false)?, scheme.base09, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base0A, true)?, scheme.base0A, RESETCOLOR, true_color(&scheme.base0A, false)?, scheme.base0A, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base0B, true)?, scheme.base0B, RESETCOLOR, true_color(&scheme.base0B, false)?, scheme.base0B, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base0C, true)?, scheme.base0C, RESETCOLOR, true_color(&scheme.base0C, false)?, scheme.base0C, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base0D, true)?, scheme.base0D, RESETCOLOR, true_color(&scheme.base0D, false)?, scheme.base0D, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base0E, true)?, scheme.base0E, RESETCOLOR, true_color(&scheme.base0E, false)?, scheme.base0E, RESETCOLOR);
                println!("{} #{} {}  {}#{}{}", true_color(&scheme.base0F, true)?, scheme.base0F, RESETCOLOR, true_color(&scheme.base0F, false)?, scheme.base0F, RESETCOLOR);
        }
    }

    Ok(())
}
