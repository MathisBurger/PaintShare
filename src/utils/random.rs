use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

// generates random token
pub fn generate_token() -> String {

    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(256)
        .map(char::from)
        .collect();
    return token;
}