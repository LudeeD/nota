use anyhow::Result;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use std::time::SystemTime;

use crate::util::{envs, filesystem};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexEntry {
    pub uid: u64,
    pub title: Option<String>,
    pub path: PathBuf,
    pub digest: String,
    pub lastupdate: SystemTime,
    pub lastexport: Option<SystemTime>,
    pub inlinks: Vec<u64>,
}

pub fn init() -> Result<Vec<IndexEntry>> {
    debug!("Initializing Index");

    match load() {
        Ok(index) => Ok(index),
        Err(_) => {
            let clean_index = vec![];
            save(&clean_index)?;
            Ok(clean_index)
        }
    }
}

pub fn load() -> Result<Vec<IndexEntry>> {
    debug!("Loading Index to memory");

    let index_path = envs::list_path();

    let bytes = filesystem::read_bytes(&index_path)?;

    match bincode::deserialize(&bytes) {
        Ok(index) => Ok(index),
        Err(_error) => Err(anyhow!("Error occurred while deserializing")),
    }
}

pub fn save(index_to_save: &[IndexEntry]) -> Result<()> {
    debug!("Saving Index");

    let index_path = envs::list_path();

    let encoded: Vec<u8> = match bincode::serialize(index_to_save) {
        Ok(bytes) => bytes,
        Err(_error) => return Err(anyhow!("Error occurred generating bincode")),
    };

    match filesystem::write_bytes(&index_path, &encoded) {
        Ok(()) => Ok(()),
        Err(_error) => Err(anyhow!("Error Occurred saving bincode")),
    }
}

pub fn list(index_to_list: &[IndexEntry]) -> Result<()> {
    println!("LIST INDEX: ");

    for entry in index_to_list {
        println!("=> {:?}", entry);
    }

    Ok(())
}