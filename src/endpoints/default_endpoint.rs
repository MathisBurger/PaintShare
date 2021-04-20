use actix_web::{Responder, web};
use serde::Serialize;
use crate::ServerData;

#[derive(Serialize)]
struct ResponseModel {
    message: String,
    status: bool,
    version: String
}

pub async fn response(data: web::Data<ServerData>) -> impl Responder {
    web::HttpResponse::Ok()
        .json(ResponseModel {
            message: "All services are running".to_string(),
            status: true,
            version: "v1.0.0-production".to_string()
        })
}