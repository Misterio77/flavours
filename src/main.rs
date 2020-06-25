#[macro_use]
extern crate clap;

use clap::App;

mod update;

fn main() {
    //Load yml file
    let yaml = load_yaml!("cli.yml");
    //Instanciate clap app from yml and add author and version info
    let matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();

    //Check which subcommand was used
    match matches.subcommand() {
//        ("apply",  Some(sub_matches)) => apply(sub_matches),
//        ("query",  Some(sub_matches)) => query(sub_matches),
        ("update", Some(sub_matches)) => update::update(sub_matches),

        _ => {},
    }
}
