use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, web, Responder};
use crate::{ServerData, middleware};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::post_comments::PostComment;
use crate::database::models::user::User;

#[derive(Deserialize)]
pub struct Query {
    pub post_id: i32,
    pub comment: String
}

/// This endpoint adds a new comment
/// to the given post
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

    PostComment::add_comment(&data.db, query.post_id, &query.comment, &user.displayname).await;

    return web::HttpResponse::Ok()
        .json(ErrorResponse { status: true, message: "Successfully posted comment".to_string() });
}