use std::fs::{File, read, create_dir};
use std::io::prelude::*;

use crate::utility::error::Upsie;

pub fn create_folder(path: &str) -> Result<(),Upsie>{
    debug!("create_folder {:?}", path);
    create_dir(path)?;
    Ok(())
}

pub fn create_file(path: &str, content: Option<&str>) -> Result<(),Upsie>{
    debug!("create_file {:?}", path);
    let mut file = File::create(path)?;
    match content {
        Some(content) => file.write_all(content.as_bytes())? ,
        None =>  {}
    };
    Ok(())
}

pub fn read_bytes(path: &str) -> Result<Vec<u8>, Upsie> {
    debug!("read bytes {:?}", path);
    Ok(read(path)?)
}

pub fn write_bytes(path: &str, bytes: &Vec<u8>) -> Result<(), Upsie>{
    debug!("write bytes {:?}", path);
    File::create(path)?.write_all(bytes)?;
    Ok(())
}