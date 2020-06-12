use crate::utility::{error::Upsie, filesystem};
use crate::service::structure;

use serde::Deserialize;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Configs{
    editor: Option<String>,
}

pub fn init_config_file() -> Result<(), Upsie>{

    let config_path = structure::configs_path()?;

    filesystem::create_file(&config_path, None)
}