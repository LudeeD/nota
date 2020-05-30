use crate::service::structure;
use crate::service::index;
use crate::service::configs;
use crate::service::book;

use std::{
    env::{temp_dir, var},
    fs::File,
    io::Read,
    path::PathBuf,
    process::Command,
};

pub fn get_nota_folder() -> PathBuf {
    structure::nota_dir_path()
}

pub fn get_index_path() -> PathBuf {
    structure::nota_index_path()
}

pub fn get_configs_path() -> PathBuf {
    structure::nota_configs_path()
}

pub fn index_print() {
    let index = index::NotaIndex::new(&get_index_path());
    println!("{:?}", index);
}

pub fn index_clean() {
    index::NotaIndex::init(&get_index_path());
    info!("Index was cleared")
}

pub fn book_generate() {
    book::generate();
}

pub fn init_nota_folder() {
    structure::init_structure();
    index::NotaIndex::init(&get_index_path());
    configs::init_config_file(&get_configs_path());
}

pub fn add_nota(nota_name: &str) {
    info!("add_nota {}", nota_name);
    let index_path = get_index_path();

    let mut index = index::NotaIndex::new(&index_path);

    let next_uid = index.get_next_uid();

    // add file to index
    index.add_new_nota(nota_name, next_uid);

    // create file <uid>.md 
    let new_nota_path = structure::add_nota(&next_uid.to_string(), nota_name);

    index.save(&index_path);

    // opens the editor
    let editor = var("EDITOR").unwrap();
    Command::new(editor)
        .arg(&new_nota_path)
        .status()
        .expect("Something went wrong");
        let mut editable = String::new();

    File::open(new_nota_path)
        .expect("Could not open file")
        .read_to_string(&mut editable);

    println!("File content:\n{}", editable);
}