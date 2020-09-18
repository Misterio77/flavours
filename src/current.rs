use std::fs;
use std::path;

use anyhow::{anyhow, Context, Result};

fn get_current_scheme(dir: &path::Path) -> Result<String> {
    // File that stores last used scheme
    let file_path = &dir.join("lastscheme");
    // Try to open it
    let scheme = fs::read_to_string(file_path)
        .with_context(|| "Failed to read last scheme file. Try applying first.")?
        .split_whitespace()
        .collect();

    if scheme == "" {
        Err(anyhow!(
            "Failed to read last scheme from file. Try applying first."
        ))
    } else {
        Ok(scheme)
    }
}

pub fn current(base_dir: &path::Path, _verbose: bool) -> Result<()> {
    println!("{}", get_current_scheme(base_dir)?);
    Ok(())
}
