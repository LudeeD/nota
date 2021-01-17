use std::env;
use std::path::{PathBuf};

use anyhow::{Context, Result};

pub fn set_export_folder(export_folder: &str) {
    env::set_var("NOTA_EXPORT_FOLDER", export_folder);
}

pub fn magic_folder() -> String { env::var("NOTA_MAGIC_FOLDER").expect("this should exist") }

pub fn nota_folder() -> String { env::var("NOTA_FOLDER").expect("this should exist") }

pub fn export_folder() -> String { env::var("NOTA_EXPORT_FOLDER").expect("this should exist") }

//pub fn links_folder() -> String { nota_folder() }

pub fn list_path() -> String { env::var("NOTA_LIST_PATH").expect("set_envs() should ensures this exist")}

pub fn index_path() -> String { env::var("NOTA_INDEX_PATH").expect("set_envs() should ensures this exist") }

pub fn configs_path() -> String { env::var("NOTA_CONFIGS_PATH").expect("set_envs() should ensures this exist") }

pub fn setup() -> Result<()> {

    let folder = env::current_dir()?.canonicalize()?;

    env::set_var("NOTA_FOLDER", &folder.to_str()
        .with_context(|| format!("Failed to set env"))?);

    const MAGIC_FOLDER_NAME : &str = ".nota";

    let mut magic_folder = folder;
    
    magic_folder.push(MAGIC_FOLDER_NAME);

    debug!("NOTA folder {:?}", magic_folder);
    env::set_var("NOTA_MAGIC_FOLDER", magic_folder.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut list_path = PathBuf::from(&magic_folder);
    list_path.push("list");
    list_path.set_extension("nota");
    debug!("List Path {:?}", list_path);
    env::set_var("NOTA_LIST_PATH", list_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut index_path = PathBuf::from(&magic_folder);
    index_path.push("index");
    index_path.set_extension("nota");
    debug!("Index Path {:?}", index_path);
    env::set_var("NOTA_INDEX_PATH", index_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    let mut configs_path = PathBuf::from(&magic_folder);
    configs_path.push("config");
    configs_path.set_extension("toml");

    debug!("Configs Path {:?}", configs_path);
    env::set_var("NOTA_CONFIGS_PATH", configs_path.to_str()
        .with_context(|| format!("Failed to set env"))?);

    //let mut export_folder = PathBuf::from(&nota_dir);
    //export_folder.push(magic_folder_name);
    //export_folder.push("export");

    //debug!("Setting NOTA_EXPORT_FOLDER env");
    //env::set_var("NOTA_EXPORT_FOLDER", export_folder.to_str()
    //    .with_context(|| format!("Failed to set env"))?);

    let mut links_folder = PathBuf::from(&magic_folder);
    links_folder.push("links");

    debug!("Setting NOTA_LINKS_FOLDER env");
    env::set_var("NOTA_LINKS_FOLDER", links_folder.to_str()
        .with_context(|| format!("Failed to set env"))?);

    return Ok(())
}