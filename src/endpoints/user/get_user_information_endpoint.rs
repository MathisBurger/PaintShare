use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;
use crate::endpoints::models::user_model::UserModel;

#[derive(Deserialize)]
pub struct Query {
    pub user_id: Option<i32>,
    pub username: Option<String>
}

/// This endpoint sends all public
/// data about the given user identified
/// by its id or username, but not both at the
/// same time
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

    let user_id_exists = match &query.user_id {
        Some(t) => true,
        None => false
    };
    let username_exists = match &query.username {
        Some(t) => true,
        None => false
    };
    if user_id_exists && username_exists || !user_id_exists && !username_exists {
        return web::HttpResponse::BadRequest()
            .json(ErrorResponse { status: false, message: "Only one parameter can be supplied at the same time to identify a user".to_string() })
    }

    let mut user: (bool, User) = (false, User::new());

    if user_id_exists {
        user = match usr.get_user_by_id(query.user_id.unwrap(), &data.db).await {
            Ok(u) => (true, u),
            Err(e) => (false, usr)
        };
    } else if username_exists {
        user = match usr.get_user_by_username(query.username.as_ref().unwrap(), &data.db).await {
            Ok(u) => (true, u),
            Err(e) => (false, usr)
        };
    }

    if !user.0 {
        return web::HttpResponse::BadRequest().json(ErrorResponse { status: false, message: "No user found identified by given user_id".to_string() })
    }

    return web::HttpResponse::Ok()
        .json(UserModel::parse_model(user.1));

}