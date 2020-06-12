#[macro_use]
extern crate clap;
use clap::App;

#[macro_use]
extern crate log;

extern crate simple_logger;

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
        match nota::init_nota_folder() {
            Ok(()) => info!("NOTA initialized"),
            Err(err) => error!("{}", err)
        };
    }

    if let Some(matches_index) = matches.subcommand_matches("index") {
        if matches_index.is_present("print") {
            nota::index_print();
        }

        if matches_index.is_present("clean") {
            match nota::index_clean(){
                Ok(()) => info!("NOTA Index Cleaned"),
                Err(err) => error!("{}", err)
            }
        }

        info!("Done !");
    }

    if let Some(matches_new) = matches.subcommand_matches("new") {
        let name = matches_new.value_of("NAME").unwrap();
        match nota::add_nota(name){
            Ok(())  => info!("Nota {} created", name),
            Err(err)=> error!("{}", err) 
        }
    }

    if let Some(_matches_book) = matches.subcommand_matches("book") {
        match nota::book_generate(){
            Ok(())  => info!("NOTA Book created"),
            Err(err)=> error!("{}", err) 
        }
    }

}
