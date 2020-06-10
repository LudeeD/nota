use serde::{Serialize, Deserialize};
use bincode;

use crate::utility::filesystem;

use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NotaIndex {
    uid: u64,
    lines: Vec<String>,
}

impl NotaIndex {

    pub fn init(index_path: &PathBuf) {
        let index = NotaIndex{uid: 0, lines: vec![]};
        index.save(index_path)
    }

    pub fn new(index_file: &PathBuf) -> NotaIndex {
        let bytes = match filesystem::read_bytes(index_file) {
            Ok(bytes) => bytes,
            Err(error) => panic!(format!("new_NotaIndex error reading bytes from file {}", error))        
        };
        match bincode::deserialize(&bytes) {
            Ok(index) => index,
            Err(error) => panic!(format!("new_NotaIndex error creating index bytes {}", error))
        }
    }

    pub fn save(&self, index_file: &PathBuf) {
        let encoded : Vec<u8> = match bincode::serialize(self) {
            Ok(bytes) => bytes,
            Err(_error) => panic!(format!("save_NotaIndex error generating bincode"))
        };

        match filesystem::write_bytes(index_file, &encoded) {
            Ok(()) => (),
            Err(_error) => panic!(format!("save_NotaIndex error saving bincode"))
        }
    }

    pub fn get_next_uid(&mut self) -> u64 {

        let new_uid = self.uid + 1;

        self.uid = new_uid;

        new_uid
    }

    pub fn add_new_nota(&mut self, name: &str, uid: u64) {
        let new_entry = String::from(format!("{};{}", uid, name));

        self.lines.push(new_entry);
    }
}