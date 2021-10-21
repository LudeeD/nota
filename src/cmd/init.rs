use clap::Clap;
use log::{debug, error, info};
use std::env;
use std::path::{Path, PathBuf};

use nota::NotaBuilder;

#[derive(Clap)]
pub struct InitCommand {
    /// The folder where you want to initialize NOTA
    /// (defaults to create in the current folder)
    #[clap(long)]
    folder: Option<String>
}

pub fn execute(command: InitCommand) {
    debug!("demo");

    let path = match command.folder {
        Some(arg) => {
            let p = Path::new(&arg);
            if p.is_relative() {
                env::current_dir().unwrap().join(p)
            } else {
                p.to_path_buf()
            }
        },
        None => {
            env::current_dir().unwrap()
        }
    };

    println!("Initializing NOTA in folder {:?}", &path);

    let builder = NotaBuilder::new(path);

    builder.init();

    builder.save();

}