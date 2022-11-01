use anyhow::{anyhow, Result};
use std::path::Path;

use crate::find::find_templates;

/// List subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours' base data dir
/// * `config_dir` - flavours' config dir
/// * `verbose` - Should we be verbose? (unused)
/// * `lines` - Should we print each scheme on its own line?
pub fn list(
    patterns: Vec<&str>,
    base_dir: &Path,
    config_dir: &Path,
    _verbose: bool,
    lines: bool,
) -> Result<()> {
    let mut templates = Vec::new();
    for pattern in patterns {
        let found_templates = find_templates(pattern, base_dir, config_dir)?;
        for found_template in found_templates {
            templates.push(found_template
                           .strip_prefix(base_dir)
                           .map_or_else(
                               |_| found_template.strip_prefix(config_dir),
                               |path| path.strip_prefix("base16/"),
                               )
                           .map_err(|_| anyhow!("Couldn't get template name"))?
                           .to_str()
                           .ok_or_else(|| anyhow!("Couldn't convert name"))?
                           .replacen("templates/", "", 2)
                           .replace(".mustache","")
                           );
        }
    }
    templates.sort();
    templates.dedup();

    if templates.is_empty() {
        return Err(anyhow!("No matching template found"));
    };

    for template in &templates {
        // Print template
        print!("{}", template);
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
