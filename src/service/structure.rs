use crate::utility::filesystem;

use std::path::Path;

pub fn init_structure(folder_name: &str){
    let nota_path = Path::new(folder_name).join("nota");
    let archive_path = nota_path.join("archive");
    let index_path = nota_path.join("index.md");

    info!("init_structure/create_main_folder - {:?}", nota_path);
    let path_str = match nota_path.as_os_str().to_str() {
        Some(path_string) => path_string,
        None => {panic!("There was a problem")},
    };
    filesystem::create_folder(path_str).unwrap();

    info!("init_structure/create_archive_folder - {:?}", archive_path);
    let archive_path_str = match  archive_path.as_os_str().to_str() {
        Some(path_str) => path_str,
        None => {panic!("There was a problem")},
    };
    filesystem::create_folder(archive_path_str).unwrap();   

    info!("init_structure/create_index_file - {:?}", index_path);
    let index_path_str = match  index_path.as_os_str().to_str() {
        Some(path_str) => path_str,
        None => {panic!("There was a problem")},
    };
    filesystem::create_file(index_path_str).unwrap();   

    
}