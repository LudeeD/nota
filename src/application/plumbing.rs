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
    index::NotaIndex::init(&get_index_path());
}

pub fn add_nota(nota_name: &str) {
    info!("add_nota {}", nota_name);
    let index_path = get_index_path();

    let mut index = index::NotaIndex::new(&index_path);

    let next_uid = index.get_next_uid();

    // add file to index
    index.add_new_nota(nota_name, next_uid);

    // create file <uid>.md 
    structure::add_nota(&next_uid.to_string(), nota_name);


    index.save(&index_path);
}