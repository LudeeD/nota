use crate::service::structure;
use crate::service::index;

use std::path::PathBuf;

pub fn get_nota_folder() -> PathBuf {
    structure::nota_dir_path()
}

pub fn get_index_path() -> PathBuf {
    structure::nota_index_path()
}

pub fn init_nota_folder() {
    structure::init_structure();
    index::init_index(&get_index_path());
}

pub fn add_nota(nota_name: &str) {
    info!("add_nota {}", nota_name);

    // get next uid
    let next_uid = index::get_next_uid(&get_index_path());

    // create file <uid>.md 
    structure::add_nota(&next_uid.to_string())

    // add file to index
}