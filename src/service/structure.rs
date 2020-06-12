use crate::utility::filesystem;
use crate::utility::error::Upsie;

use std::path::{PathBuf};
use std::env;

pub fn main_folder() -> Result<String, Upsie> {
    Ok(env::var("NOTA_FOLDER")?)
}

pub fn book_folder() -> Result<String, Upsie> {
    Ok(env::var("NOTA_BOOK_FOLDER")?)
}

pub fn index_path() -> Result<String, Upsie> {
    Ok(env::var("NOTA_INDEX_FOLDER")?)
}

pub fn configs_path() -> Result<String, Upsie> {
    Ok(env::var("NOTA_CONFIGS_FOLDER")?)
}

pub fn set_envs() -> Option<u8> {

    debug!("Fetching NOTA_FOLDER env");
    let nota_dir = env::var("NOTA_FOLDER").expect("NOTA_FOLDER env should be set");

    let mut index_path = PathBuf::from(&nota_dir);
    index_path.push("index");
    index_path.set_extension("nota");

    debug!("Setting NOTA_INDEX_PATH env");
    env::set_var("NOTA_INDEX_PATH", index_path.to_str()?);

    let mut configs_path = PathBuf::from(&nota_dir);
    configs_path.push("configs");
    configs_path.set_extension("toml");

    debug!("Setting NOTA_CONFIGS_PATH env");
    env::set_var("NOTA_CONFIGS_PATH", configs_path.to_str()?);

    let mut book_folder = PathBuf::from(&nota_dir);
    book_folder.push("book");

    debug!("Setting NOTA_BOOK_FOLDER env");
    env::set_var("NOTA_BOOK_FOLDER", book_folder.to_str()?);

    Some(0x00)
}

pub fn init_structure() -> Result<(), Upsie> {

    let path = main_folder()?;
    let mut path = PathBuf::from(&path);

    if path.exists() && path.is_dir() {
        info!("init_structure/create_main_folder nota folder already exists");
        return Ok(())
    }

    info!("init_structure/create_main_folder - {:?}", path);
    if let Some(path) = path.to_str(){
        filesystem::create_folder(path)?;
    };

    info!("init_structure/create_archive_folder - {:?}", path);
    path.push("archive");
    if let Some(path) = path.to_str(){
        filesystem::create_folder(path)?;
    };
    
    Ok(())
}

pub fn add_nota(nota_uid: &str, title: &str) -> Result<PathBuf , Upsie>{

    let new_nota_path = main_folder()?;
    let mut new_nota_path = PathBuf::from(new_nota_path);

    new_nota_path.push(nota_uid);
    new_nota_path.set_extension("md");

    if let Some(path) = new_nota_path.to_str(){
        filesystem::create_file(path, Some(&title))?;
    }

    // let title = format!("#  {}", title);

    return Ok(new_nota_path);
}