use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, web, Responder};
use crate::{ServerData, middleware};
use crate::database::models::posts::Post;
use crate::endpoints::models::error_model::ErrorResponse;
use crate::endpoints::models::post_model::PostModel;
use crate::database::models::post_comments::PostComment;
use crate::database::models::post_likes::PostLikes;

#[derive(Deserialize)]
pub struct Query {
    post_id: i32
}

#[derive(Serialize)]
struct Response {
    pub post: PostModel,
    pub comments: Vec<PostComment>,
    pub likes: Vec<PostLikes>
}


/// This endpoint returns the general data about the
/// requested post, identified by it`s id.
/// It sends all data in one body for better
/// usability on the web
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

        let comments = PostComment::get_all_comments_of_post(&data.db, post.1.id).await;
        let likes = PostLikes::get_all_likes_of_post(&data.db, post.1.id).await;

        return web::HttpResponse::Ok()
            .json(Response {
                post: PostModel::parse_model(post.1),
                comments,
                likes
            });

    }
}