use actix_web::{Responder, web};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseModel {
    message: String,
    status: bool,
    version: String
}

pub async fn response() -> impl Responder {
    web::HttpResponse::Ok()
        .json(ResponseModel {
            message: "All services are running".to_string(),
            status: true,
            version: "v1.0.0-production".to_string()
        })
}