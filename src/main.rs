use anyhow::{anyhow, Context, Result};
use std::path;

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
        return completions::completions(matches.value_of("completions"));
    };

    // Flavours data directory
    let flavours_dir = match matches.value_of("directory") {
        // User supplied
        Some(argument) => path::Path::new(argument)
            .canonicalize()
            .with_context(|| "Invalid data directory supplied")?,
        // Use default path instead
        None => dirs::data_dir()
            .ok_or_else(|| anyhow!("Error getting default data directory"))?
            .join("flavours"),
    };

    // Flavours config file
    let flavours_config = match matches.value_of("config") {
        // User supplied
        // Make it canonical, then PathBuf (owned path)
        Some(argument) => path::Path::new(argument)
            .canonicalize()
            .with_context(|| "Invalid config file supplied")?,
        // Use default file instead
        None => dirs::config_dir()
            .ok_or_else(|| anyhow!("Error getting default config directory"))?
            .join("flavours")
            .join("config.toml"),
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
            apply::apply(patterns, &flavours_dir, &flavours_config, verbose)
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
        _ => Err(anyhow!("No valid subcommand specified")),
    }
}
