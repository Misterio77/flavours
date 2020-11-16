use std::path;
use anyhow::{anyhow, Result};
use glob::glob;
use path::{PathBuf, Path};

/// Find function
///
/// * `pattern` - Which pattern to use
/// * `schemes_dir` - Schemes directory
pub fn find(pattern: &str, schemes_dir: &Path) -> Result<Vec<PathBuf>> {
    let dir = schemes_dir
        .to_str()
        .ok_or_else(|| anyhow!("Unable to convert path"))?;

    let pattern = format!("{}/*/{}.y*ml", dir, pattern);
    let matches = glob(&pattern)?;

    let mut found = Vec::new();
    for element in matches {
        found.push(element?);
    }

    Ok(found)
}
