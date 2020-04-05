use std::fs;
use std::io;

pub fn create_folder(path: &str) -> Result<(),io::Error>{
    debug!("create_folder {}", path);
    let _ = match fs::create_dir(path) {
        Ok(()) => return Ok(()),
        Err(error) => return Err(error)
    };
}

pub fn create_file(path: &str) -> Result<(),io::Error>{
    debug!("create_file {}", path);
    let _ = match fs::File::create(path) {
        Ok(_) => return Ok(()),
        Err(error) => return Err(error)
    };   
}

pub fn write() {
    println!("write")
}

pub fn read() {
    println!("read")
}