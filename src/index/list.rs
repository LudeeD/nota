use serde::{Serialize, Deserialize};
use bincode;
use anyhow::Result;

use std::convert::TryInto;

use crate::util::{envs, filesystem};

pub fn init() -> Result<()> {
    let clean_index = NotaList{entries: vec![]};

    clean_index.save();

    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ListEntry {
    uid: u64,
    title: String,
    file_path: String,
    contents_digest: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NotaList {
    entries: Vec<ListEntry>,
}

impl NotaList {

    pub fn new() -> Result<NotaList> {

        let list_path = envs::list_path();

        debug!("list path: {}", list_path);

        let bytes = filesystem::read_bytes(&list_path)?;

        match bincode::deserialize(&bytes) {
            Ok(index) => Ok(index),
            Err(_error) => Err(anyhow!("Error occurred while deserializing")),
        }
    }

    pub fn save(&self) -> Result<()> {

        let list_path = envs::list_path();
        
        debug!("list path: {}", list_path);

        let encoded : Vec<u8> = match bincode::serialize(self) {
            Ok(bytes) => bytes,
            Err(_error) => return Err(anyhow!("Error occurred generating bincode"))
        };

        match filesystem::write_bytes(&list_path, &encoded) {
            Ok(()) => Ok(()),
            Err(_error) => Err(anyhow!("Error Occurred saving bincode"))
        }
    }

    pub fn get_next_uid(&mut self) -> Result<u64> {
        let number_of_entries : u64 = (self.entries.len() + 1).try_into()?;
        Ok(number_of_entries)
    }

    pub fn add_new_nota(&mut self, title : Option<String>, file_path: String, contents_digest: String) -> Result<()> {

        let uid = self.get_next_uid()?;

        debug!("New entry {:?} {:?} {:?} {:?}", uid, title, file_path, contents_digest);

        let title = title.expect("TODO remove expect");

        let entry = ListEntry{uid, title, file_path, contents_digest};

        Ok(())

    }
}