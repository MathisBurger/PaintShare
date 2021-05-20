use actix_web::{web, Responder, HttpRequest, cookie::Cookie};
use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, Duration};
use crate::ServerData;
use crate::database::models::{user::User};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::refresh_token::RefreshToken;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ops::Add;
use actix_web::cookie::SameSite;


#[derive(Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String
}

/// This endpoint try`s to login the
/// user and returns an error,
/// if it failed. If the login was successful
/// it returns a set-cookie header containing
/// the refresh token
pub async fn response(
    req: web::Json<Request>,
    data: web::Data<ServerData>
) -> impl Responder {

    let mut usr = User::new();
    usr.displayname = req.username.clone();
    usr.password = req.password.clone();

    if !usr.check_user_existance(&data.db).await {

        web::HttpResponse::BadRequest()
            .json(ErrorResponse {
                status: false,
                message: "This user does not exist".to_string()
            })
    } else {

        if usr.check_login(&data.db).await {

            let token = RefreshToken::create_new(&data.db, &usr).await;

            let cookie = Cookie::build("refreshToken", &token.token)
                .expires(OffsetDateTime::from(SystemTime::now()
                    // UTC
                    .add(Duration::new(7200, 0))
                    // expire time
                    .add(Duration::new(432000, 0))
                ))
                .path("/")
                .secure(false)// set to false for production
                .http_only(true)
                .finish();

            web::HttpResponse::Ok()
                .cookie(cookie)
                .finish()
        } else {

            web::HttpResponse::BadRequest()
                .json(ErrorResponse {
                    status: false,
                    message: "Login credentials are wrong".to_string()
                })
        }
    }
}