#[macro_use]
extern crate log;
extern crate serde;
extern crate bincode;
extern crate comrak;
extern crate handlebars;
extern crate pulldown_cmark;
extern crate walkdir;
#[macro_use]
extern crate anyhow;

mod configs;
mod exporter;
mod index;
mod links;
mod parser;
mod util;
//mod database;
//mod filesystem;
//mod error;

//use error::Upsie;
use data_encoding::HEXUPPER;
use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use std::time::SystemTime;

use index::list::IndexEntry;
use anyhow::Result;
use std::convert::TryInto;

pub fn init_envs() {
    util::envs::setup().expect("This should not fail");
}

pub fn assert_nota_folder() -> bool {
    PathBuf::from(util::envs::magic_folder()).is_dir()
}

pub fn read_confs() {
    configs::read().expect("This should not fail");
}

pub fn demo() {
    println!("wtf");
}

/// The init command used by the CLI
/// the command will initialize a NOTA folder in the folder defined with the environment variable NOTA_FOLDER
pub fn command_init() -> bool {
    // Create NOTA main Folder
    let path = util::envs::magic_folder();
    let path = PathBuf::from(&path);

    if path.exists() && path.is_dir() {
        return false;
    } else if let Some(path) = path.to_str() {
            info!("Magic NOTA folder in - {:?}", path);
            util::filesystem::create_folder(path).expect("This should not fail 😢")
    }

    // Creates a config file
    match configs::init() {
        Ok(_) => info!("Configurations ready!"),
        Err(e) => error!("Configurations not ready {}", e),
    }

    match links::init() {
        Ok(_) => info!("Links ready!"),
        Err(e) => error!("Links not ready {}", e),
    }

    match index::list::init() {
        Ok(_) => info!("Index List ready!"),
        Err(e) => error!("Index List not ready {}", e),
    }

    true
}

pub fn command_new(_new_nota_name: Option<&str>) {

}

fn add_nota(add_path: PathBuf) -> Vec<IndexEntry> {
    if !add_path.is_file() { error!("Not a file! Not adding anything"); return vec![]; }

    let mut add_path = add_path.canonicalize().expect("Path canonicalization failed");

    debug!("{:?}", add_path);
    let nota_folder = util::envs::nota_folder();
    debug!("{:?}", nota_folder);

    if ! add_path.starts_with(&nota_folder) {
        debug!("File not in NOTA folder, copying...");
        let file_name = add_path.file_name().expect("File with no name?"); 
        let mut new_file = PathBuf::from(&nota_folder);
        new_file.push(file_name);
        new_file.set_extension("md");
        File::create(&new_file).expect("Failed to create new file");
        info!{"New file created {:?}", new_file};
        fs::copy(&add_path, &new_file).expect("Failed to copy information");
        add_path = new_file;
    }

    info!("Adding File {:?}", &add_path);

    let input = File::open(&add_path).expect("Error opening file");    
    let reader = BufReader::new(input);
    let digest = util::filesystem::sha256_digest(reader).expect("TODO remove expects | create digest");
    let hex_digest = HEXUPPER.encode(digest.as_ref());
    let info = parser::parse(&add_path).unwrap();
    let info = info.as_ref();

    let based_path = add_path.strip_prefix(nota_folder).expect("TODO remove");

    let new_entry = index::list::IndexEntry {
        uid: 0,
        title: Some(String::from(&info.title)),
        path: based_path.to_path_buf(),
        digest: hex_digest,
        lastupdate: SystemTime::now(),
        lastexport: None,
        inlinks: Vec::new()
    };

    vec![new_entry]
}

fn add_folder(in_folder: PathBuf) -> Vec<IndexEntry>{
    // add each markdown file to nota
    let folder = match fs::read_dir(in_folder) {
        Ok(folder) => folder,
        Err(_) => {
            info!("Something Went terribly wrong");
            return vec!();
        }
    };

    folder
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().unwrap() == "md")
        .map(|entry| entry.path().canonicalize())
        .filter_map(Result::ok)
        .map(add_nota)
        .filter(|entry| !entry.is_empty())
        .flatten()
        .collect()
}

/// Move file to the NOTA location
/// 
pub fn command_add(add_path: PathBuf) {
    let mut list = index::list::load().expect("Failed to load index");

    let mut new_uid : u64 = list.len().try_into().unwrap();

    let mut entries = match add_path.is_dir() {
        true => add_folder(add_path),
        false => add_nota(add_path)
    };

    for entry in entries.iter_mut() {
        new_uid += 1;
        entry.uid = new_uid;
    }

    list.append(&mut entries);

    index::list::save(&list).expect("Failed to save index after adding");
}

pub fn command_update() {
    let mut list = index::list::load().expect("Failed to load index");

    let mut remove_positions = vec![];

    for (index, entry) in list.iter_mut().enumerate() {
        let in_file = &entry.path;
        let file = File::open(&in_file);
        if file.is_err() {
            remove_positions.push(index);
        }else{
            let input = file.expect("");
            let modified_time = input
                .metadata()
                .expect("File with no metadata")
                .modified()
                .expect("File with no modified data");
        
            if modified_time > entry.lastupdate {
                info!("Updating File {:?}", &in_file);
                let reader = BufReader::new(input);
                let digest = util::filesystem::sha256_digest(reader).expect("TODO remove expects | create digest");
                let hex_digest = HEXUPPER.encode(digest.as_ref());
                let info = parser::parse(&in_file).unwrap();
                let info = info.as_ref();

                entry.title = Some(String::from(&info.title));
                entry.digest = hex_digest;
                entry.lastupdate = SystemTime::now();
            }
        };
    }

    for position in remove_positions {
        list.remove(position);
    }

    index::list::save(&list).expect("Failed to save index after update");
}

pub fn command_list() {
    let index = index::list::load().expect("TODO remove expect");

    index::list::list(&index).expect("TODO remove expect");
}

pub fn command_export(input: Option<String>, outfolder: Option<String>, templates: Option<String>) -> Result<()> {
    debug!("Export command input {:?} outfolder {:?}", input, outfolder);

    let index = index::list::load().expect("Failed to read index");

    let outfolder = outfolder.unwrap_or_else(|| "./export".to_string());
    let templates = templates.unwrap_or_else(|| "./templates".to_string());

    exporter::init(outfolder, templates)?;

    exporter::export_registered(&index)?;

    Ok(())
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
