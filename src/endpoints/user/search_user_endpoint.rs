use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::{ServerData, middleware, utils};
use crate::endpoints::models::error_model::ErrorResponse;
use crate::database::models::user::User;
use crate::endpoints::models::user_model::UserModel;

#[derive(Deserialize)]
pub struct Query {
    pub searchword: String
}

#[derive(Serialize)]
struct Response {
    status: bool,
    message: String,
    user: Vec<UserModel>
}

pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Query>
) -> impl Responder {

    let verification = middleware::validate_access_token(&req);

    if !verification.0 {
        return web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() });
    }

    return web::HttpResponse::Ok()
        .json(Response {
            status: true,
            message: "Successfully queried all available users with similar names".to_string(),
            user: UserModel::parse_models(User::search_user(&data.db, &query.searchword).await)
        });
}