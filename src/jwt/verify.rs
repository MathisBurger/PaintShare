use hmac::{Hmac, NewMac};
use jwt::{VerifyWithKey, Error};
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::utils::enviroment_handler::load_param;
use actix_web::HttpRequest;


/// This function consumes the the accessToken
/// and validates it. If it is valid, it also returns
/// the data body
pub fn verify_token(token: &String) -> (bool, BTreeMap<String, String>) {

    let key: Hmac<Sha256> = Hmac::new_varkey(load_param("HMAC_SECRET").as_bytes()).unwrap();
    let data: Result<BTreeMap<String, String>, Error> = (&token.to_owned()[..]).verify_with_key(&key);

    match data {
        Ok(map) => (true, map),
        Err(err) => (false, BTreeMap::new())
    }
}