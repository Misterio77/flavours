use anyhow::{anyhow, Context, Result};
use dirs::{data_dir, preference_dir};
use std::env;
use std::path::Path;

use flavours::operations::{apply, build, current, generate, info, list, update};
use flavours::scheme::Scheme;
use flavours::{completions, cli};

use std::fs::{create_dir_all, write};
fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    // Completetions flag
    if matches.is_present("completions") {
        return completions::completions(matches.value_of("completions"));
    };

    // Flavours data directory
    let flavours_dir = match matches.value_of("directory") {
        // User supplied
        Some(argument) => Path::new(argument)
            .canonicalize()
            .with_context(|| "Invalid data directory supplied on argument")?,
        // If not supplied
        None => {
            // Try to get from env var
            match env::var("FLAVOURS_DATA_DIRECTORY") {
                Ok(path) => Path::new(&path)
                    .canonicalize()
                    .with_context(|| "Invalid data directory supplied on env var")?,
                // Use default instead
                Err(_) => data_dir()
                    .ok_or_else(|| anyhow!("Error getting default data directory"))?
                    .join("flavours"),
            }
        }
    };

    // Flavours config file
    let flavours_config = match matches.value_of("config") {
        // User supplied
        Some(path) => Path::new(path)
            .canonicalize()
            .with_context(|| "Invalid config file supplied on argument")?,
        // If not supplied
        None => {
            // Try to get from env var
            match env::var("FLAVOURS_CONFIG_FILE") {
                Ok(path) => Path::new(&path)
                    .canonicalize()
                    .with_context(|| "Invalid config file supplied on env var")?,
                // Use default instead
                Err(_) => preference_dir()
                    .ok_or_else(|| anyhow!("Error getting default config directory"))?
                    .join("flavours")
                    .join("config.toml"),
            }
        }
    };

    // Should we be verbose?
    let verbose = matches.is_present("verbose");

    if verbose {
        println!("Using directory: {:?}", flavours_dir);
        println!("Using config file: {:?}", flavours_config);
    };

    // Check which subcommand was used
    match matches.subcommand() {
        Some(("current", _)) => current::current(&flavours_dir, verbose),

        Some(("apply", sub_matches)) => {
            //Get search patterns
            let patterns = match sub_matches.values_of("pattern") {
                Some(content) => content.collect(),
                //Defaults to wildcard
                None => vec!["*"],
            };
            let light = sub_matches.is_present("light");
            let from_stdin = sub_matches.is_present("stdin");
            apply::apply(patterns, &flavours_dir, &flavours_config, light, from_stdin, verbose)
        }

        Some(("build", sub_matches)) => {
            // Get file paths
            let scheme_file = sub_matches
                .value_of("scheme")
                .ok_or_else(|| anyhow!("You must specify a scheme file"))?;
            let template_file = sub_matches
                .value_of("template")
                .ok_or_else(|| anyhow!("You must specify a template file"))?;
            build::build(Path::new(scheme_file), Path::new(template_file))
        }

        Some(("list", sub_matches)) => {
            let patterns = match sub_matches.values_of("pattern") {
                Some(content) => content.collect(),
                //Defaults to wildcard
                None => vec!["*"],
            };
            let lines = sub_matches.is_present("lines");
            list::list(patterns, &flavours_dir, verbose, lines)
        }

        Some(("update", sub_matches)) => {
            let operation = sub_matches
                .value_of("operation")
                .ok_or_else(|| anyhow!("Invalid operation"))?;
            update::update(operation, &flavours_dir, verbose)
        }

        Some(("info", sub_matches)) => {
            let patterns = match sub_matches.values_of("pattern") {
                Some(content) => content.collect(),
                //Defaults to wildcard
                None => vec!["*"],
            };
            let raw = sub_matches.is_present("raw");
            info::info(patterns, &flavours_dir, raw)
        }

        Some(("generate", sub_matches)) => {
            let slug = sub_matches.value_of("slug").unwrap_or("generated").into();
            let name = sub_matches.value_of("name").unwrap_or("Generated").into();
            let author = sub_matches.value_of("author").unwrap_or("Flavours").into();

            let image = match sub_matches.value_of("file") {
                Some(content) => Path::new(content)
                    .canonicalize()
                    .with_context(|| "Invalid image file supplied"),
                None => Err(anyhow!("No image file specified")),
            }?;

            let mode = match sub_matches.value_of("mode") {
                Some("dark") => Ok(generate::Mode::Dark),
                Some("light") => Ok(generate::Mode::Light),
                _ => Err(anyhow!("No valid mode specified")),
            }?;

            let to_stdout = sub_matches.is_present("stdout");

            let colors = generate::generate(&image, mode, verbose)?;
            let scheme = Scheme {
                slug,
                name,
                author,
                colors,
            };

            if to_stdout {
                print!("{}", scheme.to_string()?);
            } else {
                let path = flavours_dir
                    .join("base16")
                    .join("schemes")
                    .join("generated");
                if !path.exists() {
                    create_dir_all(&path)
                        .with_context(|| format!("Couldn't create directory {:?}", &path))?;
                }
                let file_path = &path.join(format!("{}.yaml", scheme.slug));
                write(file_path, scheme.to_string()?)
                    .with_context(|| format!("Couldn't write scheme file at {:?}", path))?;
            }
            Ok(())
        }
        _ => Err(anyhow!("No valid subcommand specified")),
    }
}
