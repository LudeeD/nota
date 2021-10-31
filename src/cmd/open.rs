use clap::Clap;
use log::{debug, error, info};
use std::env;

use nota::NotaBuilder;

#[derive(Clap)]
pub struct OpenCommand { }

pub fn execute(_command: OpenCommand) {
    debug!("Open command");

    let path = env::current_dir().unwrap();

    let builder = NotaBuilder::from_path(path);

    let mut index = builder.output_folder;
    index.push("index.html");
    opener::open(index).expect("TODO");

}