use actix_web::{Responder, HttpRequest, web, HttpResponse, Error};
use crate::ServerData;
use crate::middleware;
use serde::{Serialize, Deserialize};
use crate::endpoints::error_model::ErrorResponse;
use crate::database::models::posts::Post;
use crate::utils::storage;
use actix_web::web::Bytes;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use serde_json::to_string;

#[derive(Deserialize)]
pub struct Request {
    pub comment: String,
    pub tags: String
}


/// This endpoints adds support for
/// posting new posts with images
/// checks if file extension is allowed
/// and saves the file to the local storage
/// it also adds the new post to the database
pub async fn response(
    req: HttpRequest,
    data: web::Data<ServerData>,
    query: web::Query<Request>,
    mut payload: Multipart
) -> Result<HttpResponse, Error> {

    let validation = middleware::validate_access_token(&req);

    if !validation.0 {

        Ok(web::HttpResponse::Unauthorized().json(ErrorResponse { status: false, message: "invalid access token".to_string() }))
    } else {

        // checks if file was sent via multipart/form-data
        while let Ok(Some(mut field)) = payload.try_next().await {

            let content_type = field.content_disposition().unwrap();
            let filetype = content_type.get_filename().unwrap().split(".").collect::<Vec<&str>>().last().unwrap().to_string();

            // checks if file has allowed extension
            let filetype_allowed = match filetype.clone().as_str() {
                "jpeg" => true,
                _ => false
            };


            if !filetype_allowed {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {status: false, message: "Unallowed filetype".to_string() }));
            }


            let filename = storage::generate_post_path();
            let filepath = format!("./data/posts/{}.{}", sanitize_filename::sanitize(&filename), filetype);
            let given_dest = String::from(&filepath);

            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();

            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }

            // add post to database
            Post::create_new(
            &data.db,
            validation.1.get("user_id").unwrap().parse().unwrap(),
            &query.comment,
            &query.tags,
            given_dest
            ).await;

            return Ok(web::HttpResponse::Ok()
                .json(ErrorResponse {status: true, message: "Successfully uploaded new image".to_string()}));
        }


        return Ok(web::HttpResponse::BadRequest()
            .json(ErrorResponse {status: false, message: "Missing file in multipart request".to_string()}));
    }
}

