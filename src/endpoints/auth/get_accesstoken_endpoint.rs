use actix_web::{web, Responder, HttpRequest, cookie::Cookie, HttpMessage};
use serde::{Serialize, Deserialize};
use crate::utils::request_utils;
use crate::ServerData;
use crate::database::models::refresh_token::RefreshToken;
use crate::jwt;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::models::user::User;

#[derive(Deserialize)]
pub struct Query {
    username: String
}

#[derive(Serialize)]
struct Response {
    token: String,
    deadline: i64
}


/// This endpoint returns a signed accessToken depending
/// on the refresh token given as a cookie
/// It returns status code 400 if signing is not
/// successful
pub async fn response(
    req: web::HttpRequest,
    data: web::Data<ServerData>,
    info: web::Query<Query>
) -> impl Responder {


    let cookie = request_utils::get_cookie(req, "refreshToken");

    if !cookie.0 {
        println!("Nono");
        return web::HttpResponse::Unauthorized().finish();
    }


    let tkn = RefreshToken {
        id: 0,
        username: info.username.clone(),
        token: cookie.1.split(";").collect::<Vec<&str>>()[0].to_string(),
        deadline: chrono::NaiveDateTime::from_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, 0)
    };

    if !tkn.check_existence(&data.db).await {
        println!("wanna reock");
        web::HttpResponse::Unauthorized().finish()
    } else {

        let user = User::new().get_user_by_username(&info.username, &data.db).await.unwrap();
        let deadline = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64) + 300;
        let token = jwt::sign::sign(user.user_id, deadline);

        web::HttpResponse::Ok()
            .json(Response { token, deadline })
    }
}