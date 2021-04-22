use sodiumoxide::crypto::pwhash::argon2id13;
use std::time::Instant;

// This function hashes the password
// using the argon2id algorithm
pub fn hash(pwd: &str) -> (String, argon2id13::HashedPassword) {

    sodiumoxide::init().unwrap();

    let hash = argon2id13::pwhash(
        pwd.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE
    ).unwrap();

    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    (texthash, hash)
}

// This functions verifies the hash and
// returns true, if both hashes are equal
pub fn verify(hash: &String, pwd: &String) -> bool {

    let mut padded = [0u8; 128];

    hash
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, val)| {
            padded[i] = val.clone();
        });

    sodiumoxide::init().unwrap();

    match argon2id13::HashedPassword::from_slice(&padded) {
        Some(hp) => argon2id13::pwhash_verify(&hp, pwd.as_bytes()),
        _ => false,
    }
}