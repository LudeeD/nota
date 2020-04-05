#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate log;
extern crate simple_logger;


mod application;
mod service;
mod utility;

fn main() {
    debug!("Log initialized");
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.occurrences_of("verbose") {
        1 => { println!("Log Level: Error"); simple_logger::init_with_level(log::Level::Error).unwrap();},
        2 => { println!("Log Level: Warn" ); simple_logger::init_with_level(log::Level::Warn).unwrap(); },
        3 => { println!("Log Level: Info" ); simple_logger::init_with_level(log::Level::Info).unwrap(); },
        4 => { println!("Log Level: Debug"); simple_logger::init_with_level(log::Level::Debug).unwrap(); },
        5 => { println!("Log Level: Trace"); simple_logger::init_with_level(log::Level::Trace).unwrap(); },
        _ => { println!("Log Level: Info" ); simple_logger::init_with_level(log::Level::Info).unwrap(); }
    }

    if let Some(matches_init) = matches.subcommand_matches("init") {

        let folder = matches_init.value_of("FOLDER").unwrap_or("");
        println!("Init nota folder in: {}", folder);
        application::plumbing::init_nota_folder(folder);

    }

    if let Some(matches_new) = matches.subcommand_matches("new") {

        let folder = matches_new.value_of("NAME").unwrap();
        println!("Name for new nota: {}", folder);

    }

    // Same as previous examples...
}
