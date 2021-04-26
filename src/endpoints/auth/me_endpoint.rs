use actix_web::{web, Responder, HttpRequest, cookie::Cookie, HttpMessage, HttpResponse};
use crate::jwt::verify;
use std::time::{SystemTime, UNIX_EPOCH};

/// This endpoint is made for test purposes
/// you can use it to test, if your accessToken is valid
/// it returns the jwt content if it is valid. Otherwise
/// it will send a http 400 status code.
pub async fn response(req: HttpRequest) -> impl Responder {

    let validation = crate::middleware::validate_access_token(&req);

    if validation.0 {

        web::HttpResponse::Ok().json(validation.1)
    } else {

        web::HttpResponse::BadRequest().finish()
    }
}