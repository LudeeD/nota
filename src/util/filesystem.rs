use std::fs::{File, read, create_dir};
use std::io::prelude::*;
use std::io::Error;
use ring::digest::{Context, Digest, SHA256};

use anyhow::Result;

pub fn create_folder(path: &str) -> Result<(),Error>{
    debug!("create_folder {:?}", path);
    create_dir(path)?;
    Ok(())
}

/// Will Create a new file, overwriting if there is already one present
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

pub fn write_bytes(path: &str, bytes: &[u8]) -> Result<(), Error>{
    debug!("write bytes {:?}", path);
    File::create(path)?.write_all(bytes)?;
    Ok(())
}


pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}