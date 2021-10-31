use anyhow::{Error, Result};
use log::{debug, error, info};
use std::fs::{self, File};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;
use std::io::Write;
use std::fs::OpenOptions;

pub mod index;

mod renderer;

use index::NotaIndex;
use renderer::Renderer;

#[derive(Serialize, Deserialize, Debug)]
pub struct NotaBuilder {
    pub root: PathBuf,

    pub magic_folder: PathBuf,

    pub output_folder: PathBuf
}

impl NotaBuilder {

    pub fn new(root: PathBuf) -> NotaBuilder {
        let mut magic_folder = root.clone();
        magic_folder.push(".nota");
        let mut output_folder = root.clone();
        output_folder.push("output");

        NotaBuilder {
            root, magic_folder, output_folder
        }
    }

    pub fn from_path(mut path: PathBuf) -> NotaBuilder {
        path.push(".nota");
        path.push("config");

        debug!("Reading conf from file {:?}", path);
        let file = fs::read_to_string(path)
            .expect("Something went wrong reading the file");

        toml::from_str(&file).expect("TODO")
    }

    pub fn init(&self) -> Result<()> {
        info!("Creating a new NOTA enabled folder");

        match fs::create_dir(&self.magic_folder) {
            Ok(_) => debug!("Magic folder: {:?}", &self.magic_folder),
            Err(_)=> debug!("Magic folder: {:?} already exists", &self.magic_folder)
        };

        match fs::create_dir(&self.output_folder) {
            Ok(_) => debug!("Output folder: {:?}", &self.output_folder),
            Err(_)=> debug!("Output folder: {:?} already exists", &self.output_folder)
        };

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let mut conf_location = self.magic_folder.clone();
        conf_location.push("config");
        let toml = toml::to_string(&self).unwrap();

        let mut file = File::create(conf_location).unwrap();
        file.write_all(toml.as_bytes())?;

        Ok(())
    }

    pub fn build(&self, index: &NotaIndex) {

        let r = Renderer::new();

        r.render(self, index);
    }
}
