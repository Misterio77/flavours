use anyhow::{anyhow, Result};
use palette::rgb::Rgb;
use palette::{Hsl, Yxy};
use std::collections::VecDeque;
use std::path::Path;

use crate::operations::info;

pub enum Mode {
    Light,
    Dark,
}

fn to_hex(color: Rgb) -> Result<String> {
    let (r, g, b) = color.into_components();
    let color_u8 = [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8];
    let color_hex = hex::encode(color_u8);
    Ok(color_hex)
}

fn grab_sat_luma(color: Rgb) -> (f32, f32) {
    let yxy: Yxy = Yxy::from(color);
    let (_, _, luma) = yxy.into_components();
    let hsl: Hsl = Hsl::from(color);
    let (_, saturation, _) = hsl.into_components();
    (saturation, luma)
}

fn sum_colors(color1: Rgb, color2: Rgb, ratio: f32) -> Rgb {
    let (r1, g1, b1) = color1.into_components();
    let (r2, g2, b2) = color2.into_components();
    let result: Rgb = Rgb::from_components((
        (r2 * ratio + r1 * (1.0 - ratio)),
        (g2 * ratio + g1 * (1.0 - ratio)),
        (b2 * ratio + b1 * (1.0 - ratio)),
    ));

    result
}

fn color_pass(
    colors: &[Rgb],
    min_luma: Option<f32>,
    max_luma: Option<f32>,
    min_saturation: Option<f32>,
    max_saturation: Option<f32>,
) -> Option<Rgb> {
    let predicate = |rgb: &Rgb| {
        let (saturation, luma) = grab_sat_luma(*rgb);
        
        (max_luma == None || luma <= max_luma.unwrap())
            && (min_luma == None || luma >= min_luma.unwrap())
            && (max_saturation == None || saturation <= max_saturation.unwrap())
            && (min_saturation == None || saturation >= min_saturation.unwrap())
    };

    colors.iter().copied().find(predicate)
}

fn light_color(colors: &[Rgb], verbose: bool) -> Result<Rgb> {
    let mut passes = 1;
    // Try to find a nice light color with low saturation
    let mut light = color_pass(colors, Some(0.6), None, None, Some(0.4));

    // Try again, but now we will accept saturated colors, as long as they're very bright
    if light == None {
        passes += 1;
        light = color_pass(colors, Some(0.7), None, None, Some(0.85));
    }

    // Try again, same as first, but a little more permissive
    if light == None {
        passes += 1;
        light = color_pass(colors, Some(0.5), None, None, Some(0.5));
    }

    // Try again, but accept more saturated colors
    if light == None {
        passes += 1;
        light = color_pass(colors, Some(0.6), None, None, Some(0.85));
    }

    // Try again, but now we will accept darker colors, as long as they're not saturated
    if light == None {
        passes += 1;
        light = color_pass(colors, Some(0.32), None, None, Some(0.4));
    }

    // Try again, but now we will accept even more saturated colors
    if light == None {
        passes += 1;
        light = color_pass(colors, Some(0.4), None, None, None);
    }

    // Try again, with darker colors
    if light == None {
        passes += 1;
        light = color_pass(colors, Some(0.3), None, None, None);
    }

    // Ok, we didn't find anything usable. So let's just grab the most dominant color (we'll lighten it later)
    if light == None {
        passes += 1;
        light = colors.first().copied();
    }

    if verbose {
        println!("Passes: {}", passes);
    }

    light.ok_or_else(|| anyhow!("Failed to find colors on image"))
}

fn dark_color(colors: &[Rgb], verbose: bool) -> Result<Rgb> {
    let mut passes = 1;
    // Try to find a nice darkish color with at least a bit of color
    let mut dark = color_pass(colors, Some(0.012), Some(0.1), Some(0.18), Some(0.9));

    // Try again, but now we will accept colors with any saturations, as long long as they're dark but not very dark
    if dark == None {
        passes += 1;
        dark = color_pass(colors, Some(0.012), Some(0.1), None, None);
    }

    // Try again, but now we will accept darker colors too
    if dark == None {
        passes += 1;
        dark = color_pass(colors, None, Some(0.1), None, None);
    }

    // Ok, we didn't find anything usable. So let's just grab the most dominant color (we'll darken it later)
    if dark == None {
        passes += 1;
        dark = colors.first().copied()
    }

    if verbose {
        println!("Passes: {}", passes);
    }

    dark.ok_or_else(|| anyhow!("Failed to find colors on image"))
}

fn fix_colors(dark: Rgb, light: Rgb, mode: &Mode) -> (Rgb, Rgb) {
    match mode {
        Mode::Light => {
            let mut fg = dark;
            let mut bg = light;
            // Foreground should be pretty dark and have:
            // luma <= 0.015 && saturation <= 0.65
            let (saturation, luma) = grab_sat_luma(fg);
            if luma > 0.015 {
                let yxy: Yxy = Yxy::from(fg);
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.015));
                fg = Rgb::from(yxy);
            }
            if saturation > 0.65 {
                let hsl: Hsl = Hsl::from(fg);
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.65, l));
                fg = Rgb::from(hsl);
            }

            // Background should be light have:
            // luma >= 0.7 && saturation <= 0.12
            let (saturation, luma) = grab_sat_luma(light);
            if luma < 0.75 {
                let yxy: Yxy = Yxy::from(bg);
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.75));
                bg = Rgb::from(yxy);
            }
            if saturation > 0.12 {
                let hsl: Hsl = Hsl::from(bg);
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.15, l));
                bg = Rgb::from(hsl);
            }
            (bg, fg)
        }
        Mode::Dark => {
            let mut fg = light;
            let mut bg = dark;
            // Foreground should be light and have:
            // luma >= 0.6 && saturation <= 0.15
            let (saturation, luma) = grab_sat_luma(light);
            if luma < 0.6 {
                let yxy: Yxy = Yxy::from(fg);
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.6));
                fg = Rgb::from(yxy);
            }
            if saturation > 0.15 {
                let hsl: Hsl = Hsl::from(fg);
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.15, l));
                fg = Rgb::from(hsl);
            }
            // Background should be dark and have:
            // luma <= 0.02 && saturation <= 0.6
            let (saturation, luma) = grab_sat_luma(dark);
            if luma > 0.02 {
                let yxy: Yxy = Yxy::from(bg);
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.02));
                bg = Rgb::from(yxy);
            }
            if saturation > 0.6 {
                let hsl: Hsl = Hsl::from(bg);
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.6, l));
                bg = Rgb::from(hsl);
            }
            (bg, fg)
        }
    }
}

pub fn generate(image_path: &Path, mode: Mode, verbose: bool) -> Result<VecDeque<String>> {
    let img_buffer = image::open(image_path)?;
    let img_pixels = img_buffer.to_rgba8().into_raw();

    // Use color thief to get a palette
    let palette =
        color_thief::get_palette(img_pixels.as_slice(), color_thief::ColorFormat::Rgba, 1, 15)?;

    let mut generated_colors = {
        let mut colors: Vec<Rgb> = Vec::with_capacity(palette.len());
        // For each color in palette, convert to Rgb
        for color in palette {
            let color: Rgb = Rgb::new(
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
            );
            colors.push(color);
            if verbose {
                info::print_color(&to_hex(color)?)?;
            }
        }
        // Remove duplicates
        colors.dedup();

        colors
    };

    // Get dominant light color
    let light = light_color(&generated_colors, verbose)?;
    // Get dominant dark color
    let dark = dark_color(&generated_colors, verbose)?;

    // If verbose, print them
    if verbose {
        info::print_color(&to_hex(light)?)?;
        info::print_color(&to_hex(dark)?)?;
        println!()
    }

    // Make a few adjustments to the dominant colors
    let (background, foreground) = fix_colors(dark, light, &mode);

    // If light, white. If dark, black.
    // We'll use this to make the color spectrum
    let override_color = match &mode {
        Mode::Light => Rgb::from_components((0.0, 0.0, 0.0)),
        Mode::Dark => Rgb::from_components((1.0, 1.0, 1.0)),
    };

    // Add the main colors to the vector
    let mut colors = VecDeque::new();
    colors.push_back(to_hex(background)?);
    colors.push_back(to_hex(sum_colors(background, foreground, 0.2))?);
    colors.push_back(to_hex(sum_colors(background, foreground, 0.4))?);
    colors.push_back(to_hex(sum_colors(background, foreground, 0.6))?);
    colors.push_back(to_hex(sum_colors(background, foreground, 0.8))?);
    colors.push_back(to_hex(foreground)?);
    colors.push_back(to_hex(sum_colors(foreground, override_color, 0.15))?);
    colors.push_back(to_hex(sum_colors(foreground, override_color, 0.3))?);

    // Now for the trim colors
    for _ in 0..8 {
        let mut color = generated_colors
            .pop()
            .ok_or_else(|| anyhow!("Couldn't get accent colors from image"))?;

        // Change luma to something a bit more constant
        color = {
            // Get convert to yxy and get components
            let yxy: Yxy = Yxy::from(color);
            let (x, y, luma) = yxy.into_components();

            // Get our intended luma
            let luma = match mode {
                Mode::Light => luma.min(0.12).max(0.1),
                Mode::Dark => luma.max(0.19),
            };

            // Build yxy again and convert back to rgb
            let yxy: Yxy = Yxy::from_components((x, y, luma));
            Rgb::from(yxy)
        };
        // Add to the colors vector
        colors.push_back(to_hex(color)?);
    }

    // If verbose, print our generated colors
    if verbose {
        println!();
        for color in &colors {
            info::print_color(color)?;
        }
    }

    Ok(colors)
}
