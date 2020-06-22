#[macro_use]
extern crate clap;


use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml)
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();

    match m.subcommand() {
        ("apply",  Some(sub_m)) => {
            println!("apply");
        },
        ("query",  Some(sub_m)) => {
            println!("query");
        },
        ("update", Some(sub_m)) => {
            println!("update");
        },
        _ => {},
    }
}
