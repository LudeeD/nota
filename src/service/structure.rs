use crate::utility::filesystem;

use std::path::{Path, PathBuf};
use std::env;

fn nota_dir_string() -> String {
    // very unixy ... maybe we will need to change this later
    match env::var("NOTA_FOLDER") {
        Ok(val) => val.to_string(),   
        Err(e) => {
            let user = env::var("USER").unwrap();
            error!("Error extracting $NOTA_FOLDER {:?}", e);
            format!("/home/{}/Documents/nota",user).to_string()
        }
    }
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

pub fn add_nota(nota_uid: &str, title: &str){
    let mut new_nota_path = nota_dir_path();
    new_nota_path.push(nota_uid);
    new_nota_path.set_extension("md");

    let title = format!("#  {}", title);
    filesystem::create_file(&new_nota_path, Some(&title));
}