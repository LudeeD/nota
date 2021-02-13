use serde::{Serialize, Deserialize};
use anyhow::Result;

use std::path::{PathBuf};

use crate::util;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Configs{
    editor: Option<String>,
}

pub fn read() -> Result<()> {
    Ok(())
}

pub fn init() -> Result<()> {

    // Create configs file (.notaconfig)
    let path = util::envs::configs_path();
    let path = PathBuf::from(&path);

    if path.exists(){
        info!("Configs file (.notaconfig) already exists");
    } else if let Some(path) = path.to_str() {
            info!("Creating configs file in - {:?}", path);
            util::filesystem::create_file(path, None).expect("This should not fail :(")
    }

    Ok(())
}
