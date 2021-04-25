use actix_web::{web, Responder, HttpRequest, cookie::Cookie, HttpMessage, HttpResponse};
use crate::jwt::verify;

// This endpoint is made for test purposes
// you can use it to test, if your accessToken is valid
// it returns the jwt content if it is valid. Otherwise
// it will send a http 400 status code.
pub async fn response(req: HttpRequest) -> impl Responder {

    let header = req.headers().get("authorization");

    let exists =  match header {
        Some(val) => true,
        None => false
    };

    if exists {
        let header_value: Vec<&str> = header.unwrap().to_str().unwrap().split(" ").collect::<Vec<&str>>();

        if header_value.len() != 2 {

            web::HttpResponse::BadRequest().finish()
        } else {

            if header_value[0] == "accessToken" {

                let verification = verify::verify(&header_value[1].to_string());

                if verification.0 {

                    web::HttpResponse::Ok().json(verification.1)
                } else {

                    web::HttpResponse::BadRequest().finish()
                }
            } else {

                web::HttpResponse::BadRequest().finish()
            }
        }
    } else {

        web::HttpResponse::BadRequest().finish()
    }
}