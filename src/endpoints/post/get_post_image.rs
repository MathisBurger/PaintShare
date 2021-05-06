use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::posts::Post;
use actix_web::web::post;
use actix_files::NamedFile;
use std::io::Read;
use crate::utils::storage;

#[derive(Deserialize)]
pub struct Query {
    post_id: i32
}

/// This endpoint sends the raw image
/// of a post. It requires the post id
/// to send the image
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Query>
) -> impl Responder {

    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() })
    } else {

        let post: (bool, Post) = match Post::get_post_by_id(&data.db, query.post_id).await {
            Ok(p) => (true, p),
            Err(e) => (false, Post::new())
        };
        if !post.0 {
            return web::HttpResponse::BadRequest().json(ErrorResponse { status: false, message: "This post does not exist".to_string() })
        }

        let bin = storage::read_file_to_bytes(&post.1.storage_destination);
        web::HttpResponse::Ok()
            .body(bin)

    }
}