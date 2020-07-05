use std::path;

use anyhow::{Result, anyhow};
use glob::glob;

pub fn find(pattern: &str, schemes_dir: &path::Path) -> Result<Vec<path::PathBuf>> {
    let dir = schemes_dir
        .to_str()
        .ok_or(
            anyhow!("Unable to convert path")
        )?;

    let pattern = format!("{}/*/{}.y*ml", dir, pattern);
    let matches = glob(&pattern)?;

    let mut found = Vec::new();
    for element in matches {
        found.push(element?);
    }

    Ok(found)
}

