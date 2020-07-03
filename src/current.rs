use std::path;
use std::fs;

use::anyhow::{Result, anyhow, Context};

fn get_current_scheme(dir: &path::Path) -> Result<String> {
    //File that stores last used scheme
    let file_path = &dir.join("lastscheme");
    //Try to open it
    let scheme = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read last scheme from {:?}. Try applying one first", file_path))?.split_whitespace().collect();

    if scheme == "" {
        Err(anyhow!("Last scheme file is empty. Try applying one first"))
    } else {
        Ok(scheme)
    }
}

pub fn current(base_dir: &path::Path, _verbose: bool) -> Result<()> {
    println!("{}", get_current_scheme(base_dir)?);
    Ok(())
}
