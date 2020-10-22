use std::fs::{File, read, create_dir};
use std::io::prelude::*;
use std::io::Error;

pub fn create_folder(path: &str) -> Result<(),Error>{
    debug!("create_folder {:?}", path);
    create_dir(path)?;
    Ok(())
}

pub fn create_file(path: &str, content: Option<&str>) -> Result<(),Error>{
    debug!("create_file {:?}", path);

    let mut file = File::create(path)?;

    if let Some(content) = content {
        file.write_all(content.as_bytes())?
    }

    Ok(())
}

pub fn read_bytes(path: &str) -> Result<Vec<u8>, Error> {
    debug!("read bytes {:?}", path);
    Ok(read(path)?)
}

pub fn write_bytes(path: &str, bytes: &Vec<u8>) -> Result<(), Error>{
    debug!("write bytes {:?}", path);
    File::create(path)?.write_all(bytes)?;
    Ok(())
}