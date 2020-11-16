use anyhow::{anyhow, Result};
use palette::{luma::Luma, rgb::Rgb, Blend, Hsl};
use std::path::Path;

use crate::operations::info;

pub enum Mode {
    Light,
    Dark,
    Auto,
}

fn to_hex(color: Rgb) -> Result<String> {
    let (r, g, b) = color.into_components();
    let color_u8: Vec<u8> = vec![(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8];
    let color_hex = hex::encode(color_u8);
    Ok(color_hex)
}

fn color_pass(
    colors: &[Rgb],
    min_luma: Option<f32>,
    max_luma: Option<f32>,
    min_saturation: Option<f32>,
    max_saturation: Option<f32>,
) -> Option<Rgb> {
    let mut chosen = None;
    for color in colors {
        let color = *color;
        let color_luma: Luma = Luma::from(color);
        let (luma,) = color_luma.into_components();
        let color_hsl: Hsl = Hsl::from(color);
        let (_, saturation, _) = color_hsl.into_components();
        if (max_luma == None || luma <= max_luma.unwrap())
            && (min_luma == None || luma >= min_luma.unwrap())
            && (max_saturation == None || saturation <= max_saturation.unwrap())
            && (min_saturation == None || saturation >= min_saturation.unwrap())
        {
            chosen = Some(color);
            break;
        }
    }

    chosen
}

fn light_color(colors: &[Rgb], verbose: bool) -> Result<Rgb> {
    if verbose {
        print!("Searching best light color... ");
    }
    // Try to find a nice light color with low saturation
    if verbose {
        print!("1... ");
    }
    let mut light = color_pass(colors, Some(0.85), None, None, Some(0.4));

    // Try again, but now we will accept saturated colors, as long as they're very bright
    if light == None {
        if verbose {
            print!("2... ");
        }
        light = color_pass(colors, Some(0.9), None, None, Some(0.85));
    }

    // Try again, same as first, but a little more permissive
    if light == None {
        if verbose {
            print!("3... ");
        }
        light = color_pass(colors, Some(0.75), None, None, Some(0.5));
    }

    // Try again, but accept more saturated colors
    if light == None {
        if verbose {
            print!("4... ");
        }
        light = color_pass(colors, Some(0.8), None, None, Some(0.85));
    }

    // Try again, but now we will accept darker colors, as long as they're not saturated
    if light == None {
        if verbose {
            print!("5... ");
        }
        light = color_pass(colors, Some(0.65), None, None, Some(0.4));
    }

    // Try again, but now we will accept even more saturated colors
    if light == None {
        if verbose {
            print!("6... ");
        }
        light = color_pass(colors, Some(0.65), None, None, None);
    }

    // Try again, with darker colors
    if light == None {
        if verbose {
            print!("7... ");
        }
        light = color_pass(colors, Some(0.5), None, None, None);
    }

    // Ok, we didn't find anything usable. So let's just grab the most dominant color (we'll lighten it later)
    if light == None {
        if verbose {
            print!("Giving up.");
        }
        light = match colors.first() {
            Some(color) => Some(*color),
            None => None,
        };
    }
    if verbose {
        println!();
    }

    light.ok_or_else(|| anyhow!("Failed to find colors on image"))
}

fn dark_color(colors: &[Rgb], verbose: bool) -> Result<Rgb> {
    if verbose {
        print!("Searching best dark color...");
    }
    // Try to find a nice darkish color with at least a bit of color
    if verbose {
        print!("1... ");
    }
    let mut dark = color_pass(colors, Some(0.08), Some(0.3), Some(0.18), None);

    // Try again, but now we will accept color with less saturation
    if dark == None {
        if verbose {
            print!("2... ");
        }
        dark = color_pass(colors, Some(0.08), Some(0.3), None, None);
    }

    // Try again, but now we will accept black(ish) colors too
    if dark == None {
        if verbose {
            print!("3...");
        }
        dark = color_pass(colors, None, Some(0.3), None, None);
    }

    // Ok, we didn't find anything usable. So let's just grab the most dominant color (we'll darken it later)
    if dark == None {
        if verbose {
            print!("Giving up.");
        }
        dark = match colors.first() {
            Some(color) => Some(*color),
            None => None,
        };
    }
    if verbose {
        println!();
    }

    dark.ok_or_else(|| anyhow!("Failed to find colors on image"))
}

pub fn generate(
    image_path: &Path,
    slug: &str,
    name: &str,
    author: &str,
    base_dir: &Path,
    mode: Mode,
    verbose: bool,
) -> Result<()> {
    let img_buffer = image::open(image_path)?;
    let img_pixels = img_buffer
        .as_flat_samples_u8()
        .ok_or_else(|| anyhow!("Couldn't read provided file. Is it a valid image?"))?;
    let generated_colors =
        color_thief::get_palette(img_pixels.as_slice(), color_thief::ColorFormat::Rgb, 1, 20)?;

    let mut colors: Vec<Rgb> = Vec::new();

    if verbose {
        println!("Generated colors:");
    }
    for color in generated_colors {
        let color: Rgb = Rgb::new(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        );
        colors.push(color);
        if verbose {
            let color_luma: Luma = Luma::from(color);
            let (luma,) = color_luma.into_components();
            let color_hsl: Hsl = Hsl::from(color);
            let (_, saturation, _) = color_hsl.into_components();
            info::print_color(&to_hex(color)?)?;
            println!("luma {} | saturation {}", luma, saturation);
        }
    }
    colors.dedup();

    let dark = dark_color(&colors, verbose)?;
    info::print_color(&to_hex(dark)?)?;

    let light = light_color(&colors, verbose)?;
    info::print_color(&to_hex(light)?)?;

    let blended = Rgb::from_linear(dark.into_linear().overlay(light.into_linear()));
    info::print_color(&to_hex(blended)?)?;

    Ok(())
}
