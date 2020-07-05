use std::path;
use anyhow::{Result, anyhow, Context};

mod cli;

mod apply;
mod current;
mod list;
mod update;

mod completions;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    // Completetions flag
    if matches.is_present("completions") {
        return completions::completions(matches.value_of("completions"))
    };

    // Flavours data directory
    let flavours_dir = match matches.value_of("directory") {
        // User supplied
        Some(argument) => {
            path::Path::new(argument)
                .canonicalize()
                .with_context(|| "Invalid data directory supplied")?
                .to_path_buf()
        },
        // Use default path instead
        None => {
            dirs::data_dir()
                .ok_or(anyhow!("Error getting default data directory"))?
                .join("flavours")
        }
    };

    // Flavours config file
    let flavours_config = match matches.value_of("config") {
        // User supplied
        // Make it canonical, then PathBuf (owned path)
        Some(argument) => {
            path::Path::new(argument)
                .canonicalize()
                .with_context(|| "Invalid config file supplied")?
                .to_path_buf()
        },
        // Use default file instead
        None => {
            dirs::config_dir()
                .ok_or(anyhow!("Error getting default config directory"))?
                .join("flavours").join("config.toml")
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
        ("current", Some(_)) => {
            current::current(
                &flavours_dir,
                verbose
            )
        },

        ("apply", Some(sub_matches)) => {
            apply::apply(
                sub_matches,
                &flavours_dir,
                verbose
            )
        },

        ("list",  Some(sub_matches)) => {
            list::list(
                sub_matches,
                &flavours_dir,
                verbose
            )
        },

        ("update", Some(sub_matches)) => {
            update::update(
                sub_matches,
                &flavours_dir,
                verbose
            )
        },

        _ => Err(anyhow!("No valid subcommand specified")),
    }
}
