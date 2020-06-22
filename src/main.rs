#[macro_use]
extern crate clap;


use clap::App;

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
        //Apply subcommand
        ("apply",  Some(sub_matches)) => {
            //If pattern isn't specified, grab last applied scheme
            let pattern = sub_matches.value_of("pattern")
                          .unwrap_or_else(|| "placeholder");
            println!("{}", pattern);
        },
        //Query subcommand
        ("query",  Some(sub_matches)) => {
            //If pattern isn't specified, grab last applied scheme
            let pattern = sub_matches.value_of("pattern")
                          .unwrap_or_else(|| "placeholder");
            println!("{}", pattern);
        },
        //Update subcommand
        ("update", Some(sub_matches)) => {
            //Check which update operation was used
            //(We can safely unwrap, clap handles errors or missing arguments)
            match sub_matches.value_of("operation").unwrap() {
                //All operation
                "all" => {},
                //Lists operation
                "lists" => {},
                //Schemes
                "schemes" => {},
                //Template
                "templates" => {},
                _ => {},
            }
        },
        _ => {},
    }
}
