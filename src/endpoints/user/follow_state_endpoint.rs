use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware, utils};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;
use crate::utils::iterator::check_duplicates;

#[derive(Deserialize)]
pub struct Query {
    pub user_id: i32
}

#[derive(Serialize)]
struct Response {
    check_status: bool,
    message: String
}

/// This endpoint checks if the user
/// who owns the active session follows the
/// given user_id and returns the state as
/// boolean value
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Query>
) -> impl Responder {

    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        return web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() });
    }

    let user = User::new().get_user_by_id(
        verification.1.get("user_id").unwrap().parse::<i32>().unwrap(),
        &data.db
    ).await.unwrap();

    return web::HttpResponse::Ok()
        .json(Response {
            check_status: check_duplicates(&user.subscriptions, query.user_id),
            message: "Successfully queried follow state of the user".to_string()
        });
}