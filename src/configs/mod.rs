use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::util;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Configs{
    editor: Option<String>,
}

pub fn init() -> Result<()> {

    let configs_path = util::envs::configs_path();
    debug!("creating configs file in {:?}", configs_path);
    util::filesystem::create_file(&configs_path, None);

    Ok(())
}
