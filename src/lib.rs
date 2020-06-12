#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

extern crate bincode;

extern crate comrak;

extern crate handlebars;

extern crate walkdir;

mod service;
mod utility;

use crate::service::structure;
use crate::service::index;
use crate::service::configs;
use crate::service::book;

use crate::utility::error::Upsie;

use std::{ fs::File, io::Write };

pub fn index_print() {
    let index = index::NotaIndex::new();
    println!("{:?}", index);
}

pub fn index_clean()  -> Result<(), Upsie>{
    index::NotaIndex::init()?;
    info!("Index was cleared");
    Ok(())
}

pub fn book_generate()  -> Result<(), Upsie>{
    book::generate()?;
    Ok(())
}

pub fn init_nota_folder() -> Result<(), Upsie>{
    debug!("Init Structure");
    structure::init_structure()?;
    index::NotaIndex::init()?;
    configs::init_config_file()?;
    Ok(())
}

pub fn init_envs() {
    structure::set_envs();
}

pub fn add_nota(nota_name: &str) -> Result<(),Upsie> {
    info!("add_nota {}", nota_name);

    let mut index = index::NotaIndex::new()?;

    let next_uid = index.get_next_uid();

    // add file to index
    index.add_new_nota(nota_name, next_uid);

    // create file <uid>.md 
    let new_nota_path = structure::add_nota(&next_uid.to_string(), nota_name)?;

    //let default_text = format!("# {}", nota_name);

    let mut new_file = File::open(new_nota_path)?;

    write!(new_file,"# {}", nota_name).expect("damn");

    index.save()

    // opens the editor
    //let editor = var("EDITOR").unwrap();
    //Command::new(editor)
    //    .arg(&new_nota_path)
    //    .status()
    //    .expect("Something went wrong");
    //    let mut editable = String::new();

    //File::open(new_nota_path)
    //    .expect("Could not open file")
    //    .read_to_string(&mut editable);

    //println!("File content:\n{}", editable);
}