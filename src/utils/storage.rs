use std::fs;
use crate::utils::random::generate_token;
use std::io::Write;
use std::future::Future;
use actix_web::{Error, dev, error};
use rand::Rng;
use futures::future;

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
