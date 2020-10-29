use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use bincode;
use std::path::PathBuf;
use anyhow::Result;

use std::convert::TryInto;

use crate::util::{envs, filesystem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexEntry {
    pub uid: u64,
    pub original_title: Option<String>,
    pub file_path: PathBuf,
    pub contents_digest: String,
    pub replaced_by: Option<u64>
}

pub fn init() -> Result<Vec<IndexEntry>> {
    debug!("Initializing Index");

    let clean_index = vec![];

    save(&clean_index);

    Ok(clean_index)
}

pub fn load() -> Result<Vec<IndexEntry>> {

    debug!("Loading Index to memory");

    let index_path = envs::index_path();

    let bytes = filesystem::read_bytes(&index_path)?;

    match bincode::deserialize(&bytes) {
        Ok(index) => Ok(index),
        Err(_error) => Err(anyhow!("Error occurred while deserializing")),
    }
}

pub fn save(index_to_save: &Vec<IndexEntry>) -> Result<()> {
    debug!("Saving Index");

    let index_path = envs::index_path();

    let encoded : Vec<u8> = match bincode::serialize(index_to_save) {
        Ok(bytes) => bytes,
        Err(_error) => return Err(anyhow!("Error occurred generating bincode"))
    };

    match filesystem::write_bytes(&index_path, &encoded) {
        Ok(()) => Ok(()),
        Err(_error) => Err(anyhow!("Error Occurred saving bincode"))
    }
}

pub fn list(index_to_list: &Vec<IndexEntry>) -> Result<()> {

    println!("LIST INDEX: ");

    for entry in index_to_list {
        println!("=> {:?}", entry);
    }

    Ok(())
}

pub fn add_new_nota(index: &mut Vec<IndexEntry>, mut entry: IndexEntry) -> Result<()> {

    let new_uid = index.len().try_into().unwrap();

    entry.uid = new_uid;

    index.push(entry);

    Ok(())
}

pub fn search_for_uid(index: & Vec<IndexEntry>, uid_to_search: u64) -> Result<IndexEntry> {

    let mut entry : Option<IndexEntry> = None;

    for elem in index.iter() {

        if elem.uid == uid_to_search {

            entry = Some(elem.clone());

        }

    }

    match entry {
        Some(e) => Ok(e),
        None => Err(anyhow!("No index entry found"))
    }

}

pub fn search_for_path(index: & Vec<IndexEntry>, path_to_search: PathBuf) -> Result<IndexEntry> {

    let mut entry : Option<IndexEntry> = None;

    for elem in index.iter() {

        if elem.file_path == path_to_search {

            entry = Some(elem.clone());

        }

    }

    match entry {
        Some(e) => Ok(e),
        None => Err(anyhow!("No index entry found"))
    }

}