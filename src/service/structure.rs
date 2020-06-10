use crate::utility::filesystem;

use std::path::{ PathBuf};
use std::env;

fn nota_dir_string() -> String {
    debug!("Fetching NOTA_FOLDER env");
    env::var("NOTA_FOLDER").expect("Error getting NOTA_FOLDER env")
}

pub fn nota_dir_path() -> PathBuf {
    PathBuf::from(nota_dir_string())
}

pub fn nota_index_path() -> PathBuf{
    let mut index_path = nota_dir_path();
    index_path.push("index");
    index_path.set_extension("md");
    index_path
}

pub fn nota_configs_path() -> PathBuf{
    let mut configs_path = nota_dir_path();
    configs_path.push("configs");
    configs_path.set_extension("toml");
    configs_path
}

pub fn nota_book_folder_path() -> PathBuf{
    let mut book_folder_path = nota_dir_path();
    book_folder_path.push("book");
    book_folder_path
}

pub fn init_structure() {
    let mut path = nota_dir_path();

    if path.exists() && path.is_dir() {
        info!("init_structure/create_main_folder nota folder already exists");
        return
    }

    info!("init_structure/create_main_folder - {:?}", path);
    filesystem::create_folder(&path).unwrap();

    path.push("archive");

    info!("init_structure/create_archive_folder - {:?}", path);
    filesystem::create_folder(&path).unwrap();   
}

pub fn add_nota(nota_uid: &str, title: &str) -> PathBuf{
    let mut new_nota_path = nota_dir_path();
    new_nota_path.push(nota_uid);
    new_nota_path.set_extension("md");

    let title = format!("#  {}", title);
    filesystem::create_file(&new_nota_path, Some(&title));

    return new_nota_path;
}