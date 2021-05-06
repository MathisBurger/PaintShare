use actix_web::{web, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::ServerData;
use crate::database::models::user;
use crate::utils::hashing;
use crate::database::register;


#[derive(Deserialize)]
pub struct Request {
    pub displayname: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
struct Response {
    status: bool,
    message: String
}

/// This endpoint inserts the new user into
/// the database, if there is no account
/// existing with given username
pub async fn response(
    req: web::Json<Request>,
    data: web::Data<ServerData>
) -> impl Responder {


    let mut usr = user::User::new();
    usr.displayname = req.displayname.clone();
    usr.email = req.email.clone();
    usr.password = req.password.clone();

    if usr.check_user_existance(&data.db).await {

        web::HttpResponse::Ok()
            .json(crate::endpoints::models::error_model::ErrorResponse {
                status: false,
                message: "This username is already in use".to_string()
            })
    } else {

        usr.password = hashing::hash(&usr.password).0;
        let status = register::register(&usr, &data.db).await;
        
        if status {

            web::HttpResponse::Ok()
                .json(Response {
                    status,
                    message: "Successfully created account".to_string()
                })
        } else {
            
            web::HttpResponse::Ok()
                .json(crate::endpoints::models::error_model::ErrorResponse {
                    status,
                    message: "Error while creating user account".to_string()
                })
        }

    }



}