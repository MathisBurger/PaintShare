use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::ServerData;
use crate::database::models::user;
use crate::jwt::verify;
use crate::database::models::user::User;
use crate::utils::file;
use crate::middleware;
use crate::endpoints::error_model::ErrorResponse;

#[derive(Deserialize)]
pub struct Query {
    pub user: Option<String>
}

/// This endpoint is made for requesting profile pictures.
/// You can add the "user" request param to get the profile picture
/// of an specific user. If you leave it out, it will return your profile
/// picture. There is a default profile picture
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Query>
) -> impl Responder {


    let verification = middleware::validate_access_token(&req);

    if !verification.0 {

        web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() })
    } else {

        let user_itself = match &query.user {
            Some(T) => false,
            None => true
        };

        let mut user = User::new();

        if user_itself {

            let user_id = verification.1.get("user_id").unwrap().parse().unwrap();
            user = User::new().get_user_by_id(user_id, &data.db).await.unwrap();
        } else {
            let username = query.user.as_ref().unwrap();
            let usr  = User::new()
                .get_user_by_username(username, &data.db).await;

            let exists = match &usr {
                Ok(T) => true,
                Err(E) => false
            };

            if !exists {
                return web::HttpResponse::BadRequest().finish();
            }

            user = usr.unwrap();
        }

        if user.profile_picture == "" {
            let img_bytes = file::file_to_bytes(&"./data/default_profile_picture.jpg".to_string());

            if img_bytes.is_err() {

                return web::HttpResponse::BadRequest().finish();
            }

            return web::HttpResponse::Ok().body(img_bytes.unwrap());
        } else {
            let mut path = String::from("./data/profile_pictures/");
            path.push_str(&*user.profile_picture);
            let img_bytes = file::file_to_bytes(&path);

            if img_bytes.is_err() {

                return web::HttpResponse::BadRequest().finish();
            }

            return web::HttpResponse::Ok().body(img_bytes.unwrap());
        }
    }
}