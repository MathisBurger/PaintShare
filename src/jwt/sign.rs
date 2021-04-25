use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::utils::enviroment_handler::load_param;

// This function signs a json web token
// and returns it
pub fn sign(user_id: i32, deadline: i64) -> String {

    let key: Hmac<Sha256> = Hmac::new_varkey(load_param("HMAC_SECRET").as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id.to_string());
    claims.insert("deadline", deadline.to_string());

    claims.sign_with_key(&key).unwrap()
}