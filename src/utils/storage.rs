use std::fs;
use crate::utils::random::generate_token;
use std::io::{Write, Read};
use std::future::Future;
use actix_web::{Error, dev, error};
use rand::Rng;
use futures::future;
use std::fs::File;

/// This function generates a random filename
/// and returns the name as string
pub fn generate_post_path() -> String {
    
    let mut random = generate_token(32);
    while path_exists(&format!("./data/posts/{}.jpg", random)) {}
    return random;
}

/// This function checks wheather
/// a file exists or not and returns
/// the state as a boolean value
fn path_exists(path: &String) -> bool {
    
    let meta = fs::metadata(path);
    match meta { 
        Ok(T) => true,
        Err(E) => false
    }
}

/// This function takes bytes and writes them
/// into the file located at the
/// given destination path
pub fn write_bytes_to_file(destination: &String, data: &actix_web::web::Bytes) {
    let mut pos = 0;
    let mut buffer = fs::File::open(destination).unwrap();

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]).unwrap();
        pos += bytes_written;
    }
}

/// This function reads the binary data
/// of a file and returns it as Vec<u8>
pub fn read_file_to_bytes(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
