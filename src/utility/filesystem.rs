use std::path::{PathBuf};
use std::fs::{read, File, create_dir};
use std::io::prelude::*;
use std::io::{Error, BufReader, BufRead};

pub fn create_folder(path: &PathBuf) -> Result<(),Error>{
    debug!("create_folder {:?}", path);
    create_dir(path)
}

pub fn create_file(path: &PathBuf, content: Option<&str>) -> Result<(),Error>{
    debug!("create_file {:?}", path);
    let mut file = File::create(path)?;

    match content {
        Some(content) => { file.write_all(content.as_bytes())},
        None =>  {Ok(())}
    }
}

pub fn read_bytes(path: & PathBuf) -> Result<Vec<u8>, Error> {
    read(path)
}

pub fn write_bytes(path: & PathBuf, bytes: &Vec<u8>) -> Result<(), Error>{
    let mut f = File::create(path).expect("Demo");
    f.write_all(bytes)
}