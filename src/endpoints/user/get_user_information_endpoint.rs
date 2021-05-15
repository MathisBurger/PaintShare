use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;
use crate::endpoints::models::user_model::UserModel;

#[derive(Deserialize)]
pub struct Query {
    pub user_id: i32
}

/// This endpoint sends all public
/// data about the given
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Query>
) -> impl Responder {

    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        return web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() })
    }
    let usr = User::new();

    let user: (bool, User) = match usr.get_user_by_id(query.user_id, &data.db).await {
        Ok(u) => (true, u),
        Err(e) => (false, usr)
    };

    if !user.0 {
        return web::HttpResponse::BadRequest().json(ErrorResponse { status: false, message: "No user found identified by given user_id".to_string() })
    }

    return web::HttpResponse::Ok()
        .json(UserModel::parse_model(user.1));

}