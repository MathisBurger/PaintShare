use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware, utils};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;

#[derive(Deserialize)]
pub struct Query {
    pub user_id: i32
}

/// This endpoint removes a subscription
/// from the user. It validates all the given data
/// and returns the status and the message as
/// ErrorResponse
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
    if !User::remove_subscription_from_user(&data.db, user_id, query.user_id).await {
        return web::HttpResponse::BadRequest()
            .json(ErrorResponse { status: false, message: "Cannot remove subscription".to_string() });
    }

    User::increase_follower_of_user(&data.db, query.user_id, -1).await;
    User::increase_subscriptions_of_user(&data.db, user_id, -1).await;

    return web::HttpResponse::Ok()
        .json(ErrorResponse { status: true, message: "Successfully removed subscription".to_string() });
}