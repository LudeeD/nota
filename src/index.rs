use crate::NotaBuilder;

use anyhow::{Error, Result};
use glob::glob;
use log::{debug, error, info};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct NotaIndex {
    nota_store: HashSet<Nota>
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct Nota {
    pub path    : PathBuf,
}

impl NotaIndex {

    pub fn from_builder(builder: &NotaBuilder) -> Result<NotaIndex> {
        
        let mut index_path = builder.magic_folder.clone();

        index_path.push("index");

        if index_path.exists() {
            debug!("Loading index {:?}", index_path);
            let mut f = File::open(index_path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;

            Ok(bincode::deserialize(&buffer)?)
        } else {
            debug!("Index file not present, creating new");
            Ok(NotaIndex { nota_store: HashSet::new() })
        }
    }

    pub fn update(&mut self, builder: &NotaBuilder) {
        let path = builder.root.clone();
        let path = path.into_os_string().into_string().unwrap();
        let g = format!("{}*.md", path);
        for path in glob(&g).unwrap().filter_map(Result::ok) {
            let file_name = path.file_name().expect("TODO");
            print!("Found {:?} ", file_name);
            let new_nota = Nota {path};

            if self.nota_store.insert(new_nota) {
                println!("- New");
            } else {
                println!();
            };
        }

        self.nota_store.retain(|nota| nota.path.exists());

    }

    pub fn save_to_disk(&self, builder: &NotaBuilder) {
        debug!("saving index");

        let mut index_path = builder.magic_folder.clone();

        index_path.push("index");

        let encoded: Vec<u8> = match bincode::serialize(self) {
            Ok(bytes) => bytes,
            Err(_error) => {error!("SHAIT"); return;},
        };

        let mut file = File::create(index_path).unwrap();
        file.write_all(&encoded).expect("TODO");
    }
}
