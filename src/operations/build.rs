use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path;

use crate::scheme::Scheme;

/// Build a template
///
/// Given template base and scheme, builds the template and returns it
///
/// * `template_base` - Template base string
/// * `scheme` - Scheme structure
pub fn build_template(template_base: String, scheme: &Scheme) -> Result<String> {
    let mut built_template = template_base;
    built_template = built_template
        .replace("{{scheme-name}}", &scheme.name)
        .replace("{{scheme-author}}", &scheme.author)
        .replace("{{scheme-slug}}", &scheme.slug);

    for (name, color) in scheme.colors.iter().enumerate() {
        let hex = String::from(color).replace("#", "");
        let rgb = hex::decode(&hex)?;
        built_template = built_template
            .replace(
                //hex
                &format!("{{{{base0{:X}-hex}}}}", name),
                &hex,
            )
            .replace(
                //hex-r
                &format!("{{{{base0{:X}-hex-r}}}}", name),
                &hex[0..2],
            )
            .replace(
                //hex-g
                &format!("{{{{base0{:X}-hex-g}}}}", name),
                &hex[2..4],
            )
            .replace(
                //hex-b
                &format!("{{{{base0{:X}-hex-b}}}}", name),
                &hex[4..6],
            )
            .replace(
                //hex-bgr
                &format!("{{{{base0{:X}-hex-bgr}}}}", name),
                &format!("{}{}{}", &hex[4..6], &hex[2..4], &hex[0..2]),
            )
            .replace(
                //rgb-r
                &format!("{{{{base0{:X}-rgb-r}}}}", name),
                &format!("{}", rgb[0]),
            )
            .replace(
                //rgb-g
                &format!("{{{{base0{:X}-rgb-g}}}}", name),
                &format!("{}", rgb[1]),
            )
            .replace(
                //rgb-b
                &format!("{{{{base0{:X}-rgb-b}}}}", name),
                &format!("{}", rgb[2]),
            )
            .replace(
                //dec-r
                &format!("{{{{base0{:X}-dec-r}}}}", name),
                &format!("{:.2}", (rgb[0] as f64) / (255_f64)),
            )
            .replace(
                //dec-g
                &format!("{{{{base0{:X}-dec-g}}}}", name),
                &format!("{:.2}", (rgb[1] as f64) / (255_f64)),
            )
            .replace(
                //dec-b
                &format!("{{{{base0{:X}-dec-b}}}}", name),
                &format!("{:.2}", (rgb[2] as f64) / (255_f64)),
            )
    }
    Ok(built_template)
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
    let scheme = Scheme::from_str(scheme_contents, &slug)?;
    //Template content
    let template_content = fs::read_to_string(template_file)
        .with_context(|| format!("Couldn't read template file at {:?}.", template_file))?;

    //Template with correct colors
    let built_template = build_template(template_content, &scheme)
        .context("Couldn't replace placeholders. Check if all colors on the specified scheme file are valid.")?;

    println!("{}", built_template);
    Ok(())
}
