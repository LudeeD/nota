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
use std::path::{PathBuf};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use data_encoding::HEXUPPER;

pub fn init_envs() {
    util::envs::setup();
}

pub fn assert_nota_folder() -> bool{
    PathBuf::from(util::envs::magic_folder()).is_dir()
}

pub fn read_confs() {
   configs::read();
}

/// The init command used by the CLI
/// the command will initialize a NOTA folder in the folder defined with the environment variable NOTA_FOLDER
pub fn command_init() -> bool {

    // Create NOTA main Folder
    let path = util::envs::magic_folder();
    let path = PathBuf::from(&path);

    if path.exists() && path.is_dir() {
        return false;
    } else {
        if let Some(path) = path.to_str() {
            info!("Magic NOTA folder in - {:?}", path);
            util::filesystem::create_folder(path).expect("This should not fail 😢")
        };
    }

    // Creates a config file
    match configs::init() {
        Ok(_) => info!("Configurations ready!"),
        Err(e) => error!("Configurations not ready {}", e)
    }

    match links::init() {
        Ok(_) => info!("Links ready!"),
        Err(e) => error!("Links not ready {}", e)
    }

    match index::list::init() {
        Ok(_) => info!("Index List ready!"),
        Err(e) => error!("Index List not ready {}", e)
    }

    return true;
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

fn add_nota(in_file: PathBuf) {

    debug!("Adding File {:?}", in_file);

    let mut index = index::list::load().expect("TODO remove expects | load index");

    let file_name = in_file.file_name().unwrap();

    let input = File::open(&in_file).expect("TODO remove expects | open file input");
    let reader = BufReader::new(input);
    let digest = util::filesystem::sha256_digest(reader).expect("TODO remove expects | create digest");
    let hex_digest = HEXUPPER.encode(digest.as_ref());

    let nota_folder = util::envs::nota_folder();

    let mut new_file = PathBuf::from(nota_folder);

    new_file.push(file_name);
    new_file.set_extension("md");

    // TODO check if the file already exists
    File::create(&new_file).expect("TODO remove expects | create file");

    fs::copy(&in_file, &new_file).expect("TODO remove expects | copy file");

    let info = parser::parse(&in_file).unwrap();

    let info = info.as_ref();

    let index_entry = index::list::IndexEntry{
        uid: 0,
        original_title: Some(String::from(&info.title)),
        file_path: new_file,
        contents_digest: hex_digest,
        replaced_by: None
    };

    index::list::add_new_nota(&mut index, index_entry).expect("TODO remove expects");

    index::list::save(&index).expect("TODO remove expects | save index");
}

/// move file to the NOTA location
pub fn command_add(in_file: PathBuf) {

    let dir = if in_file.is_dir() {
        debug!("command_add arg is dir");
        match fs::read_dir(&in_file) {
            Ok(dir) => Some(dir),
            Err(_) => None
        }
    } else { None };

    match dir {
        Some(dir) => {
            debug!("Adding All files from dir: {:?}", dir);
            for entry in dir {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if entry.path().extension().unwrap() == "md" {
                                add_nota(entry.path());
                            }
                        }
                    }
                }
            }
        },
        None => {
            add_nota(in_file);
        }
    }

}

pub fn command_update() {
/*
    match index::list::init() {
        Ok(list) => {
            index::list::save(&list).expect("Couldn't save index");
            list
        }
        Err(e)   => panic!(e)
    };

    let folder = util::envs::main_folder();

    //command_add(PathBuf::from(folder));
*/
}

pub fn command_list() {
    let index = index::list::load().expect("TODO remove expect");

    index::list::list(&index);
}

pub fn command_export(input: Option<PathBuf>, outfolder: PathBuf) {
    debug!("Export command input {:?} outfolder {:?}", input, outfolder);
    let index = index::list::load().expect("TODO remove expect");

    exporter::exporter::init(&outfolder);
    exporter::exporter::export_registered(&index);
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