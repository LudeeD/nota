#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate log;

extern crate simple_logger;

use std::path::{PathBuf};

fn main() {
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
    debug!("Log initialized");

    nota::init_envs();
    debug!("Envs loaded");

    if let Some(_matches_init) = matches.subcommand_matches("init") {
        if ! nota::command_init() {
            info!("It was not possible to initialize NOTA folder, maybe this is a NOTA folder already");
            return;
        }
    }

    if ! nota::assert_nota_folder() {
        info!("Not a NOTA folder, stoping...");
        return;
    }

    nota::read_confs();
    debug!("Configuration File read");

    if let Some(matches_new) = matches.subcommand_matches("new") {
        let new_nota_name = matches_new.value_of("NAME").unwrap();
        nota::command_new(Some(new_nota_name)); 
    }

    if let Some(matches_add) = matches.subcommand_matches("add") {
        let file = matches_add.value_of("PATH").unwrap();
        let file = PathBuf::from(file);
        nota::command_add(file); 
        info!("File(s) moved and added");
        return;
    }

    if let Some(_matches_list) = matches.subcommand_matches("list") {
        nota::command_list(); 
    }

    if let Some(_matches_list) = matches.subcommand_matches("update") {
        nota::command_update(); 
    }

    if let Some(matches_export) = matches.subcommand_matches("export") {
        let file = match matches_export.value_of("PATH") {
            Some(path) => Some(PathBuf::from(path)),
            None => None
        };
        let outfolder = match matches_export.value_of("outfolder") {
            Some(path) => PathBuf::from(path),
            None => {
                println!("FUCk");
                return
            }
        };
        nota::command_export(file, outfolder); 
    }

}
