#[macro_use]
extern crate clap;
use clap::App;

mod application;
mod service;
mod utility;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Same as previous examples...
    application::plumbing::init_nota();
}
