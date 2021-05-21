use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;
use crate::utils::user_handler::Query;
use crate::utils;
use crate::database::models::posts::Post;
use crate::endpoints::models::post_model;
use crate::endpoints::models::post_model::PostModel;

#[derive(Serialize)]
struct Response {
    status: bool,
    username: String,
    posts: Vec<PostModel>
}

/// This endpoint sends the basic information
/// about all the posts of the given user
/// If no user is given, it sends the information
/// of the session owner
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Query>
) -> impl Responder {

    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() })
    } else {

        let user_req = utils::user_handler::get_user_from_request(&query, &verification, &data.db).await;
        if !user_req.0 {
            return web::HttpResponse::BadRequest().json(ErrorResponse { status: false, message: "no user given for request".to_string() })
        }

        let user = user_req.1;
        let posts = Post::get_all_posts_of_user(&data.db, user.user_id).await;
        web::HttpResponse::Ok()
            .json(Response {
                status: true,
                username: user.displayname,
                posts: post_model::PostModel::parse_models(posts)
            })
    }
}