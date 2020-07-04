use anyhow::{Result, anyhow};

mod cli;

mod apply;
mod current;
mod list;
mod update;

mod completions;

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    //Completetions flag
    if matches.is_present("completions") {
        return completions::completions(matches.value_of("completions"))
    }

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
        ("list",  Some(sub_matches)) => 
            list::list(sub_matches, &flavours_dir, verbose),
        ("update", Some(sub_matches)) => 
            update::update(sub_matches, &flavours_dir, verbose),
        ("apply", Some(sub_matches)) =>
            apply::apply(sub_matches, &flavours_dir, verbose),
        ("current", Some(_)) =>
            current::current(&flavours_dir, verbose),
        _ => Err(anyhow!("No valid subcommand specified")),
    }
}
