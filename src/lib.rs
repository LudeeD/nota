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
extern crate sled;

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
use std::path::{ Path, PathBuf};
use std::time::SystemTime;

use index::list::IndexEntry;
use anyhow::Result;
use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use sha1::{Sha1, Digest};

use std::env;

const SLED_ERROR : &str = "sled error";
const BINCODE_ERROR : &str = "bincode error";
const ENVS_ERROR : &str = "envs error";
const OTHER_ERROR : &str = "error";

use pulldown_cmark::{Parser, Event, Tag::Link, html};
use std::io::Read;

use handlebars::{to_json, Handlebars};
use std::io::prelude::*;
use std::collections::BTreeMap;

pub fn generate_nota(handlebars: &handlebars::Handlebars, db: &sled::Db, output: &PathBuf, nota: &PathBuf) {
    info!("Parsing {:?} to HTML", nota);

    let buffer: String = fs::read_to_string(&nota).expect("io error");
    let parser = Parser::new(&buffer);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);


    let mut data = BTreeMap::new();
    data.insert("body".to_string(), to_json(html_output));
    let dt = SystemTime::now();
    data.insert("lastmodified".to_string(), to_json(format!("{:?}",dt)));

    let link_tree = db.open_tree("link").expect(SLED_ERROR);
    let mut hasher = Sha1::new();
    hasher.update(nota.to_str().unwrap());
    let path_hash = hasher.finalize();

    match link_tree.get(path_hash).expect(SLED_ERROR) {
        Some(value) => {
            let decoded: Vec<PathBuf> = bincode::deserialize(&value[..]).unwrap();
            let links: Vec<String> = decoded.into_iter().map(|value| format!("<a href=\"{:?}\">{:?}</a>", value, value)).collect();
            data.insert("links".to_string(), to_json(links));
        },
        None => {
            data.insert("links".to_string(), to_json(vec!["<a>No links</a>".to_string()]));
        }
    }


    let mut output_file = File::create(output).unwrap();

    output_file.write_all(handlebars.render("entry", &data).unwrap().as_bytes()).expect(OTHER_ERROR);
}

pub fn parse_links(path: &PathBuf,  link_tree: &sled::Tree) {
    info!("Parsing for links {:?}", &path);
    let mut f = File::open(path).expect("io error");
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).expect("io error");
    let parser = Parser::new(&buffer);

    // TODO: collapse this 2 passes into only 1
    let _parser = parser.for_each(|event| 
        if let Event::Start(Link(_link_type, destination, _title)) = event {
            let dest = PathBuf::from(destination.as_ref());
            //let key_entry = path.clone().canonicalize();

            match dest.clone().canonicalize() {
                Ok(dest_canon) => {
                    let mut hasher = Sha1::new();
                    hasher.update(dest_canon.to_str().unwrap());
                    let key = hasher.finalize();
                    info!("{:?} -> {:?} [{:?}]", path, &dest, &dest_canon);

                    let _oldvalue = link_tree.fetch_and_update(&key, |value| {
                        let new_value = match value {
                            Some(value) => {
                                let mut decoded: Vec<PathBuf> = bincode::deserialize(&value[..]).unwrap();
                                let already_present = decoded.binary_search(&dest);
                                if already_present.is_err() {
                                    decoded.push(dest.to_path_buf());
                                }
                                decoded
                            },
                            None => vec![dest.to_path_buf()]
                        };

                        let bytes = bincode::serialize(&new_value).expect(BINCODE_ERROR);
                        Some(bytes)
                    });
                },
                Err(_) => ()
            }
        }
    );

}

pub fn index_nota(path: &PathBuf, db: &sled::Db) {
    let main_tree = db.open_tree("main").expect("SLED_ERROR");
    let link_tree = db.open_tree("link").expect("SLED_ERROR");

    let mut hasher = Sha1::new();
    hasher.update(path.to_str().unwrap());
    let path_hash = hasher.finalize();

    let current_time = SystemTime::now();
    let bytes = bincode::serialize(&current_time).expect(BINCODE_ERROR);
    main_tree.insert(&path_hash, bytes).expect(SLED_ERROR);

    parse_links(path, &link_tree);
}

pub fn generate() {

    let path = env::current_dir().expect(ENVS_ERROR);

    let mut full_path = path.canonicalize().expect(OTHER_ERROR);
    full_path.push(".nota");
    debug!("Open sled {:?}", &full_path);
    let tree = sled::open(full_path.to_str().unwrap()).expect(SLED_ERROR);
    full_path.pop();

    let folder = fs::read_dir(&full_path).expect(OTHER_ERROR);
    debug!("Analyze folder {:?}", &folder);

    let entries : Vec<PathBuf> = folder
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| { 
            entry.path().is_file() && 
            entry.path().extension().is_some() && 
            entry.path().extension().unwrap() == "md" })
        .map(|entry| entry.path().canonicalize())
        .filter_map(Result::ok)
        .collect();
    
    for entry_path in &entries {
        index_nota(entry_path, &tree);
    }

    full_path.push("nota.html");
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("entry", &full_path)
        .expect("damn");

    full_path.pop();
    full_path.push("output");
    for entry_path in &entries {
        let filename = entry_path.file_name().unwrap();

        let mut output_file = full_path.clone();
        output_file.push(filename);
        output_file.set_extension("html");

        generate_nota(&handlebars, &tree, &output_file, entry_path);
    };

    tree.flush().expect(SLED_ERROR);
    tree.clear().expect(SLED_ERROR);
}

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
pub fn command_init(mut init_path: PathBuf) -> bool {
    // Create NOTA main Folder
    init_path.push(".nota");

    if fs::create_dir(init_path.as_path()).is_err() { return false }

    init_path.push("nota.db");
    let tree = sled::open(init_path.to_str().unwrap()).expect("open");
    tree.insert("uid", "0");
    tree.flush();

    init_path.pop();
    init_path.push("links");
    if fs::create_dir(init_path.as_path()).is_err() { return false }

    init_path.pop();
    init_path.push("cuckoos");
    if fs::create_dir(init_path.as_path()).is_err() { return false }

    true
}

pub fn command_new(_new_nota_name: Option<&str>) {

}




fn add_nota_to_index(based_path: PathBuf, db: &sled::Db){
    let mut hasher = Sha1::new();
    hasher.update(&based_path.to_str().unwrap());
    let path_hash = hasher.finalize();

    let main_tree = db.open_tree("main").expect(SLED_ERROR);


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
        title: String::from(&info.title),
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

    let entries = match add_path.is_dir() {
        true => add_folder(add_path),
        false => add_nota(add_path)
    };

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

                entry.title = String::from(&info.title);
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
