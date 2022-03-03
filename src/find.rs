use anyhow::Result;
use glob::glob;
use path::{Path, PathBuf};
use std::path;

/// Find color schemes matching pattern in either the config dir or the data dir.
///
/// * `pattern` - Which pattern to use
/// * `base_dir` - flavours' base data dir
/// * `config_dir` - flavours' config dir
pub fn find_schemes(pattern: &str, base_dir: &Path, config_dir: &Path) -> Result<Vec<PathBuf>> {
    let config_scheme_dir = config_dir.join("schemes");
    let data_scheme_dir = base_dir.join("base16").join("schemes");
    let dir_vec = vec![config_scheme_dir, data_scheme_dir];
    let dir_vec: Vec<&str> = dir_vec.iter().filter_map(|dir| dir.to_str()).collect();

    let mut found = Vec::new();
    for dir in dir_vec {
        let glob_pattern = format!("{}/*/{}.y*ml", dir, pattern);
        let matches = glob(&glob_pattern)?;
        for element in matches {
            found.push(element?);
        }
    }
    Ok(found)
}

/// Find template file in either the config dir or the data dir.
///
/// * `template` - template
/// * `subtemplate` - subtemplate
/// * `base_dir` - flavours' base data dir
/// * `config_dir` - flavours' config dir
pub fn find_template(
    template: &str,
    subtemplate: &str,
    base_dir: &Path,
    config_dir: &Path,
) -> Result<PathBuf> {
    let template_config_file = config_dir
        .join("templates")
        .join(&template)
        .join("templates")
        .join(format!("{}.mustache", subtemplate));

    let template_data_file = base_dir
        .join("base16")
        .join("templates")
        .join(&template)
        .join("templates")
        .join(format!("{}.mustache", subtemplate));

    if template_config_file.is_file() {
        Ok(template_config_file)
    } else if template_data_file.is_file() {
        Ok(template_data_file)
    } else {
        panic!(
            "Neither {:?} or {:?} exist",
            template_config_file, template_data_file
        )
    }
}
