use std::collections::BTreeMap;
use actix_web::{HttpRequest};
use crate::jwt::verify;
use std::time::{SystemTime, UNIX_EPOCH};


/// This function validates the accessToken
/// and returns the validation status as boolean
/// If the validation was successful the Payload
/// of the token will also be returned
pub fn validate_access_token(req: &HttpRequest) -> (bool, BTreeMap<String, String>) {

    let header = req.headers().get("authorization");

    let exists =  match header {
        Some(val) => true,
        None => false
    };

    if exists {

        let header_value: Vec<&str> = header.unwrap().to_str().unwrap().split(" ").collect::<Vec<&str>>();

        if header_value.len() != 2 {

            (false, BTreeMap::new())
        } else {

            if header_value[0] == "accessToken" {

                let verification = verify::verify_token(&header_value[1].to_string());

                if verification.0 {

                    let deadline_unix: i64 = verification.1.get("deadline").unwrap().parse().unwrap();

                    if (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64) < deadline_unix {

                        (true, verification.1)
                    } else {

                        (false, BTreeMap::new())
                    }
                } else {

                    (false, BTreeMap::new())
                }
            } else {

                (false, BTreeMap::new())
            }
        }
    } else {

        (false, BTreeMap::new())
    }
}