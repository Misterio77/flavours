use std::path;

use anyhow::{anyhow, Result};

#[path = "find.rs"]
mod find;

/// List subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours base data dir
/// * `verbose` - Should we be verbose? (unused)
/// * `lines` - Should we print each scheme on its own line?
pub fn list(patterns: Vec<&str>, base_dir: &path::Path, _verbose: bool, lines: bool) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find::find(pattern, &base_dir.join("base16").join("schemes"))?;

        for found_scheme in found_schemes {
            schemes.push(String::from(
                found_scheme
                    .file_stem()
                    .ok_or_else(|| anyhow!("Couldn't get scheme name"))?
                    .to_str()
                    .ok_or_else(|| anyhow!("Couldn't convert name"))?,
            ));
        }
    }
    schemes.sort();
    schemes.dedup();

    if schemes.is_empty() {
        return Err(anyhow!("No matching scheme found"));
    };

    for scheme in &schemes {
        // Print scheme
        print!("{}", scheme);
        if lines {
            // Print newline
            println!();
        } else {
            // Print space
            print!(" ");
        }
    }
    // If we separated by spaces, print an ending newline
    if !lines {
        println!();
    }

    Ok(())
}
