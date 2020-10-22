use std::env;
use std::path::{PathBuf};

use anyhow::{Context, Result};

// TODO: do the same for the other variables
const ENV_MAIN_FOLDER : &str = "NOTA_FOLDER";

pub fn main_folder() -> String { env::var(ENV_MAIN_FOLDER).expect("set_envs() should ensures this exist") }

pub fn export_folder() -> String { env::var("NOTA_EXPORT_FOLDER").expect("set_envs() should ensures this exist") }

pub fn nota_folder() -> String { env::var("NOTA_MAGIC_FOLDER").expect("set_envs() should ensures this exist") }

pub fn links_folder() -> String { nota_folder() }

pub fn list_path() -> String { env::var("NOTA_LIST_PATH").expect("set_envs() should ensures this exist")}

pub fn index_path() -> String { env::var("NOTA_INDEX_PATH").expect("set_envs() should ensures this exist") }

pub fn configs_path() -> String { env::var("NOTA_CONFIGS_PATH").expect("set_envs() should ensures this exist") }

pub fn init() -> Result<()> {

    const magic_folder_name : &str = ".nota";

    debug!("Fetching {} env", ENV_MAIN_FOLDER);
    let nota_dir = env::var(ENV_MAIN_FOLDER).expect(&format!("{} env should be set", ENV_MAIN_FOLDER));

    let mut magic_path = PathBuf::from(&nota_dir);
    magic_path.push(magic_folder_name);
    debug!("Setting NOTA_MAGIC_FOLDER env");
    env::set_var("NOTA_MAGIC_FOLDER", magic_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut index_path = PathBuf::from(nota_folder());
    index_path.push("index");
    index_path.set_extension("nota");
    debug!("Setting NOTA_INDEX_PATH env");
    env::set_var("NOTA_INDEX_PATH", index_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut list_path = PathBuf::from(nota_folder());
    list_path.push("list");
    list_path.set_extension("nota");
    debug!("Setting NOTA_LIST_PATH env");
    env::set_var("NOTA_LIST_PATH", list_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut configs_path = PathBuf::from(&nota_dir);
    configs_path.push(magic_folder_name);
    configs_path.push("notaconfig");
    configs_path.set_extension("toml");

    debug!("Setting NOTA_CONFIGS_PATH env");
    env::set_var("NOTA_CONFIGS_PATH", configs_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut export_folder = PathBuf::from(&nota_dir);
    export_folder.push("export");

    debug!("Setting NOTA_EXPORT_FOLDER env");
    env::set_var("NOTA_EXPORT_FOLDER", export_folder.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut links_folder = PathBuf::from(&nota_dir);
    links_folder.push("notalinks");

    debug!("Setting NOTA_LINKS_FOLDER env");
    env::set_var("NOTA_LINKS_FOLDER", links_folder.to_str()
        .with_context(|| format!("Failed to set env"))?);

    return Ok(())
}