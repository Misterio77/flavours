use anyhow::{anyhow, Context, Result};
use base16_color_scheme::{Scheme, Template};
use std::fs;
use std::path;

/// Build a template
///
/// Given template base and scheme, builds the template and returns it
///
/// * `template_base` - Template base string
/// * `scheme` - Scheme structure
pub fn build_template(template_base: &str, scheme: &Scheme) -> Result<String> {
    let template = Template::new(template_base)?;
    Ok(template.render(scheme))
}

/// Build function
///
/// * `scheme_file` - Path to scheme file
/// * `template_file` - Path to template
pub fn build(scheme_file: &path::Path, template_file: &path::Path) -> Result<()> {
    //Read chosen scheme
    let scheme_contents = &fs::read_to_string(&scheme_file)
        .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;

    let slug = scheme_file
        .file_stem()
        .ok_or_else(|| anyhow!("The scheme path must contain a valid filename"))?
        .to_string_lossy();
    let mut scheme: Scheme = serde_yaml::from_str(scheme_contents)?;
    scheme.slug = slug.to_string();

    //Template content
    let template_content = fs::read_to_string(template_file)
        .with_context(|| format!("Couldn't read template file at {:?}.", template_file))?;

    let template = Template::new(template_content)?;

    //Template with correct colors
    println!("{}", template.render(&scheme));
    Ok(())
}
