use anyhow::{Result, anyhow};

mod cli;
mod update;
mod completions;
mod query;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    //Should we be verbose?
    let verbose = matches.is_present("verbose");

    //Data directory
    let data_dir = match dirs::data_dir() {
        Some(value) => value,
        None => return Err(anyhow!("Error getting data directory")),
    };
    let flavours_dir = &data_dir.join("flavours");

    if verbose { println!("Using directory: {:?}", flavours_dir) }
    //Config file
    //TODO


    //Check which subcommand was used
    match matches.subcommand() {
//        ("apply",  Some(sub_matches)) => apply(sub_matches),
        ("query",  Some(sub_matches)) => 
            query::query(sub_matches, &flavours_dir, verbose),
        ("update", Some(sub_matches)) => 
            update::update(sub_matches, &flavours_dir, verbose),
        ("completions", Some(sub_matches)) => 
            completions::completions(sub_matches),

        _ => Err(anyhow!("No valid subcommand specified")),
    }
}
