#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate bincode;
extern crate comrak;
extern crate handlebars;
extern crate walkdir;
extern crate pulldown_cmark;

#[macro_use]
extern crate anyhow;

mod util;
mod configs;
mod links;
mod index;
mod parser;
mod exporter;
//mod database;
//mod filesystem;
//mod error;

//use error::Upsie;
use std::{ fs::File, io::Read, io::Write };
use std::path::{PathBuf};

//use database::IndexEntry;
//use chrono::prelude::*;

//pub const REVERSE_LINKS_HEADING_LEVEL : &str = "###";
//pub const REVERSE_LINKS_TEXT : &str = "Reverse Links";

pub fn init_envs() {
    util::envs::init();
}

/// The init command used by the CLI
/// the command will initialize a NOTA folder in the folder defined with the environment variable NOTA_FOLDER
pub fn command_init() {

    // Initializes NOTA Folder
    let path = util::envs::main_folder();
    let mut path = PathBuf::from(&path);

    if path.exists() && path.is_dir() {
        info!("nota folder already exists");
        return 
    }

    info!("create main folder in - {:?}", path);
    if let Some(path) = path.to_str(){
        util::filesystem::create_folder(path).expect("Hum...");
    };

    // Creates a config file
    configs::init();

    // TODO maybe links should not be the one responsible for creating the .nota folder
    info!("create links folder in");
    links::init();

    info!("create index list file");
    index::list::init();
}

pub fn command_new(new_nota_name: Option<&str>) {

    //let utc_time : &str = &Utc::now().to_string();

    //let title = match new_nota_name {
    //    Some(title) => { title },
    //    None => { utc_time }
    //};

    //// Try and add the new nota to the database
    //debug!("Adding NOTA to the db. title: {} time: {:?}", title, utc_time);
    //let new_entry = IndexEntry::new(title.to_string(), utc_time.to_string());
    //database::add_index_entry(new_entry).expect("Error adding Entry");

    //// Get the respective ID
    //let id = database::get_uid_from_title(title).unwrap();
    //debug!("NOTA added to the db with id {}", id);

    //let mut nota_path = PathBuf::from(&util::main_folder());
    //nota_path.push(id.to_string());
    //nota_path.set_extension("md");

    //let content = format!("# {}\n\n\n{} {}", title, REVERSE_LINKS_HEADING_LEVEL, REVERSE_LINKS_TEXT);

    //filesystem::create_file(nota_path.to_str().unwrap(), Some(&content))
    //    .map_err(|err| {
    //        error!("Error creating a new NOTA {}", err)
    //    })
    //    .unwrap();

}

/// move file to the NOTA location
pub fn command_add(mut in_file: File) {

    //let info = parser::parse(in_file);

    //let info = info.as_ref();

    //debug!("Title: {}", info.title());

    //info.refs().into_iter().for_each(|uid| {
    //    debug!("Links to: {}", uid);
    //});

}

pub fn command_update() {
    panic!("Not Implemented")
}

pub fn command_list() {
    panic!("Not Implemented")
}

pub fn command_export() {
    exporter::exporter::init();
    exporter::exporter::export(Option::None);
}

pub fn command_agenda() {
    panic!("Not Implemented")
}

// Creates a new note in NOTA
//
// Steps:
//
//     - gets next uid for the note from index
//     - adds information to index
//     - creates the file
//     - Opens the editor ? Not Sure yet
//pub fn create_nota(nota_name: &str) -> Result<(),Upsie> {
//    info!("create_nota {}", nota_name);
//
//    let mut index = index::NotaIndex::new()?;
//
//    let next_uid = index.get_next_uid();
//
//    // add file to index
//    index.add_new_nota(nota_name, next_uid);
//
//    // create file <uid>.md 
//    let new_nota_path = structure::create_nota(&next_uid.to_string(), nota_name)?;
//
//    //let default_text = format!("# {}", nota_name);
//
//    let mut new_file = File::open(new_nota_path)?;
//
//    write!(new_file,"# {}", nota_name).expect("damn");
//
//    index.save()
//
//    // opens the editor
//    //let editor = var("EDITOR").unwrap();
//    //Command::new(editor)
//    //    .arg(&new_nota_path)
//    //    .status()
//    //    .expect("Something went wrong");
//    //    let mut editable = String::new();
//
//    //File::open(new_nota_path)
//    //    .expect("Could not open file")
//    //    .read_to_string(&mut editable);
//
//    //println!("File content:\n{}", editable);
//}
//
//pub fn add_nota(path: &str) -> Result<(),Upsie> {
//    init_envs();
//
//    info!("add_nota source {}", path);
//
//    let mut index = index::NotaIndex::new()?;
//
//    let next_uid = index.get_next_uid();
//    info!("next_uid {}", next_uid);
//
//    // create file <uid>.md 
//    let nota_name = structure::add_nota(&next_uid.to_string(), path)?;
//    info!("nota_name {}", nota_name);
//
//    // add file to index
//    index.add_new_nota(&nota_name, next_uid);
//    index.save()?;
//
//    Ok(())
//}