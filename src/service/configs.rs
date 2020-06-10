use crate::utility::filesystem;

use std::path::{PathBuf};

use serde::Deserialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Configs{
    editor: Option<String>,
}

pub fn init_config_file(path: &PathBuf){
    filesystem::create_file(path, None).expect("Something went Wrong");
}