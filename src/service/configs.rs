use crate::utility::filesystem;

use std::path::{Path, PathBuf};
use std::env;

use serde::Deserialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Configs{
    editor: Option<String>,
}

pub fn init_config_file(path: &PathBuf){
    filesystem::create_file(path, None);
}