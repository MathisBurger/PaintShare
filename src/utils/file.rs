use std::fs::File;
use std::io::Read;

/// This function reads the bytes of
/// the file with the given path
/// and returns it as a result
pub fn file_to_bytes(name: &String) -> std::io::Result<Vec<u8>> {

    let mut f = File::open(name)?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}