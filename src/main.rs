#[macro_use]
extern crate clap;
use clap::App;

fn apply() {}
fn query() {}
fn update() {}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
}
