use serde::{Serialize, Deserialize};
use bincode;

use crate::utility::filesystem;

use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct NotaIndex {
    uid: u64,
    lines: Vec<String>,
}

pub fn init_index(index_path: &PathBuf) {
    write_index(&NotaIndex{uid: 0, lines: vec![]}, index_path)
}

fn write_index(index: &NotaIndex, index_file: &PathBuf){
    let encoded : Vec<u8> = bincode::serialize(index).unwrap();
    filesystem::write_bytes(index_file, &encoded).expect("Damn")
}

fn read_index(index_file: &PathBuf) -> NotaIndex {
    let bytes = filesystem::read_bytes(index_file).expect("Damn");
    let decoded : NotaIndex = bincode::deserialize(&bytes).expect("Dman");
    decoded
}

pub fn get_next_uid(index_path: &PathBuf) -> u64 {

    let mut index = read_index(index_path);

    let new_uid = index.uid + 1;

    index.uid = new_uid;

    write_index(&index, &index_path);

    return new_uid;
}