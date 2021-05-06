use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

/// generates random token with
/// given size
pub fn generate_token(size: usize) -> String {

    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    return token;
}