use std::path;

use anyhow::{Result, anyhow};

#[path = "find.rs"]
mod find;

pub fn list(patterns: Vec<&str>, base_dir: &path::Path, _verbose: bool, lines: bool) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {        
        let found_schemes = find::find(
            pattern,
            &base_dir.join("base16").join("schemes")
        )?;

        for found_scheme in found_schemes {
            schemes.push(
                String::from(
                    found_scheme
                    .file_stem()
                    .ok_or(
                        anyhow!("Couldn't get scheme name")
                    )?
                    .to_str()
                    .ok_or(
                        anyhow!("Couldn't convert name")
                    )?
                )
            );
        }
    }
    schemes.sort();
    schemes.dedup();

    for scheme in schemes {
        // Print scheme
        print!("{}", scheme);
        if lines {
            // Print newline
            println!("");
        } else {
            // Print space
            print!(" ");
        }
    }
    // If we separated by spaces, print an ending newline
    if !lines { println!(""); }

    Ok(())
}
