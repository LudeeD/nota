use clap::Clap;
use log::{debug, error, info};
use std::env;
use std::path::{Path, PathBuf};

use nota::NotaBuilder;

use nota::index::NotaIndex;

#[derive(Clap)]
pub struct BuildCommand { }

pub fn execute(_command: BuildCommand) {
    debug!("Build command");

    let path = env::current_dir().unwrap();

    let builder = NotaBuilder::from_path(path);

    let mut index = NotaIndex::from_builder(&builder).unwrap();

    index.update(&builder);
    index.save_to_disk(&builder);

    builder.build(&index);


}