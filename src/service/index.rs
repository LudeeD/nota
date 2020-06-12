use serde::{Serialize, Deserialize};
use bincode;


use crate::utility::filesystem;
use crate::utility::error::Upsie;
use crate::service::structure;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NotaIndex {
    uid: u64,
    lines: Vec<String>,
}

impl NotaIndex {

    pub fn init() -> Result<(), Upsie> {
        let index = NotaIndex{uid: 0, lines: vec![]};
        index.save()?;
        Ok(())
    }

    pub fn new() -> Result<NotaIndex, Upsie> {

        let index_path = structure::index_path()?;

        let bytes = filesystem::read_bytes(&index_path)?;

        match bincode::deserialize(&bytes) {
            Ok(index) => Ok(index),
            Err(_error) => Err(Upsie::new("Error Occurred While Deserializing")),
        }
    }

    pub fn save(&self) -> Result<(),Upsie> {

        let index_path = structure::index_path()?;
        
        let encoded : Vec<u8> = match bincode::serialize(self) {
            Ok(bytes) => bytes,
            Err(_error) => return Err(Upsie::new("Error Occurred generating bincode"))
        };

        match filesystem::write_bytes(&index_path, &encoded) {
            Ok(()) => Ok(()),
            Err(_error) => Err(Upsie::new("Error Occurred saving bincode"))
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