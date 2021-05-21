use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware, utils};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;

#[derive(Deserialize)]
pub struct Query {
    pub user_id: i32
}

/// This endpoint adds a new subscription
/// to the user. It changes the feed behaviour
/// of the user, because it affects the content
/// that will be shown to the subscriber
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Json<Query>
) -> impl Responder {

    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        return web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() });
    }

    let user_id: i32 = verification.1.get("user_id").unwrap().parse().unwrap();
    if !User::add_subscription_to_user(&data.db, user_id, query.user_id).await {
        return web::HttpResponse::BadRequest()
            .json(ErrorResponse { status: false, message: "Cannot add subscription".to_string() });
    }

    User::increase_follower_of_user(&data.db, query.user_id, 1).await;
    User::increase_subscriptions_of_user(&data.db, user_id, 1).await;

    return web::HttpResponse::Ok()
        .json(ErrorResponse { status: true, message: "Successfully added subscription".to_string() });
}