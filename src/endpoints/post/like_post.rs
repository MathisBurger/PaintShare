use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, web, Responder};
use crate::{ServerData, middleware};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::post_likes::PostLikes;
use crate::database::models::user::User;
use crate::database::models::posts::Post;

#[derive(Deserialize)]
pub struct Query {
    pub post_id: i32
}

/// This endpoint increases the number of total
/// likes of an specific post, given in the request,
/// by one, if the user has not liked the post
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Json<Query>
) -> impl Responder {
    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        return web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() });
    }

    let mut user = User::new();
    let user_id: i32 = verification.1.get("user_id").unwrap().parse().unwrap();
    user = user.get_user_by_id(user_id, &data.db).await.unwrap();

    let post_exists: (bool, Post) = match Post::get_post_by_id(&data.db, query.post_id).await {
        Ok(post) => (true, post),
        Err(e) => (false, Post::new())
    };

    if !post_exists.0 {
        return web::HttpResponse::BadRequest()
            .json(ErrorResponse { status: false, message: "This post does not exist".to_string() });
    }

    if PostLikes::user_liked_post(&data.db, query.post_id, &user.displayname).await {
        return web::HttpResponse::Ok()
            .json(ErrorResponse { status: false, message: "You already liked this post".to_string() });
    }

    Post::increase_likes(&data.db, &post_exists.1, 1).await;
    PostLikes::insert_like(&data.db, query.post_id, &user.displayname).await;

    return web::HttpResponse::Ok()
        .json(ErrorResponse { status: true, message: "Post liked".to_string() });
}