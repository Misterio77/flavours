use anyhow::{anyhow, Context, Result};
use base16_color_scheme::{
    scheme::{RgbColor, RgbColorFormatter},
    Scheme,
};
use calm_io::stdoutln;
use std::fs::read_to_string;
use std::path::Path;

use crate::find::find_schemes;

fn true_color(hex_color: &str, background: bool) -> Result<String> {
    let rgb = hex::decode(hex_color)?;

    let code = if background { 48 } else { 38 };

    Ok(format!("\x1b[{};2;{};{};{}m", code, rgb[0], rgb[1], rgb[2]))
}

pub fn print_color(color: &str) -> Result<()> {
    const RESETCOLOR: &str = "\x1b[0m";
    match stdoutln!(
        "{} #{} {}  {}#{}{}",
        true_color(color, true)?,
        color,
        RESETCOLOR,
        true_color(color, false)?,
        color,
        RESETCOLOR
    ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

pub fn print_color_rgb(color: RgbColor) -> Result<()> {
    use base16_color_scheme::template::color_field::{Format, Hex};
    use std::fmt::{self, Display, Formatter};

    struct TrueColor(RgbColor, bool);

    impl Display for TrueColor {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let RgbColor([r, g, b]) = self.0;
            let code = if self.1 { 48 } else { 38 };
            write!(f, "\x1b[{code};2;{r};{g};{b}m")
        }
    }

    const RESETCOLOR: &str = "\x1b[0m";

    let true_color_fg = TrueColor(color, true);
    let true_color_bg = TrueColor(color, false);

    let color = RgbColorFormatter {
        color,
        format: Format::Hex(Hex::Rgb),
    };
    match stdoutln!("{true_color_fg} #{color} {RESETCOLOR}  {true_color_bg}#{color}{RESETCOLOR}",) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

/// Info subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours base data dir
/// * `verbose` - Should we be verbose? (unused)
/// * `color` - Should we print with colors?
pub fn info(patterns: Vec<&str>, base_dir: &Path, config_dir: &Path, raw: bool) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find_schemes(pattern, base_dir, config_dir)?;

        for found_scheme in found_schemes {
            schemes.push(found_scheme);
        }
    }
    schemes.sort();
    schemes.dedup();

    if schemes.is_empty() {
        return Err(anyhow!("No matching scheme found"));
    };

    let mut first = true;
    for scheme_file in schemes {
        if first {
            first = false;
        } else {
            match stdoutln!() {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            }?;
        }
        let scheme_slug = scheme_file
            .file_stem()
            .ok_or_else(|| anyhow!("Couldn't get scheme name."))?
            .to_str()
            .ok_or_else(|| anyhow!("Couldn't convert scheme file name."))?;
        let scheme_contents = read_to_string(&scheme_file)
            .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;

        let mut scheme: Scheme = serde_yaml::from_str(&scheme_contents)?;
        scheme.slug = scheme_slug.to_string();

        match stdoutln!(
            "{} ({}) @ {}",
            scheme.scheme,
            scheme.slug,
            scheme_file.to_string_lossy()
        ) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        }?;

        match stdoutln!("by {}", scheme.author) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        }?;

        if raw {
            for (_, &color) in scheme.colors.iter() {
                use base16_color_scheme::template::color_field::{Format, Hex};
                match stdoutln!(
                    "#{}",
                    RgbColorFormatter {
                        color,
                        format: Format::Hex(Hex::Rgb)
                    }
                ) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                }?;
            }
        } else {
            for (_, &color) in scheme.colors.iter() {
                print_color_rgb(color)?;
            }
        }
    }

    Ok(())
}
