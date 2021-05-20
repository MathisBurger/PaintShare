extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix_web::{HttpServer, App, web, middleware as actix_middleware, HttpResponse};
use dotenv::dotenv;
use sqlx::{mysql, Pool, MySql};
use actix_cors::Cors;
use actix_files::Files;

mod database;
mod utils;
mod endpoints;
mod jwt;
mod middleware;

// This struct is sent
// to every endpoint as
// serverside parameter
pub struct ServerData {
    pub db: Pool<MySql>
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    pretty_env_logger::init();


    dotenv().ok();

    let conn = mysql::MySqlPool::connect(&utils::enviroment_handler::load_param("DATABASE_URL"))
    .await.expect("Cannot create database connection");


    sqlx::migrate!("./migrations/")
        .run(&conn)
        .await.expect("Cannot run migrations");


    database::service::init_tables(&conn).await;


    HttpServer::new(move || {
        App::new()
        .data(ServerData {db: conn.clone()})
        .wrap(actix_middleware::Logger::default())
        .wrap(Cors::default()
            .allowed_origin("http://localhost:3000") // for production purposes
            .allow_any_method()
            .allow_any_origin()
            .allow_any_header()
            .expose_any_header()
            .supports_credentials()
            .max_age(3600)
        )
        .route("/api", web::get().to(endpoints::default_endpoint::response))
        // auth
        .route("/api/user/register", web::post().to(endpoints::register_endpoint::response))
        .route("/api/auth/login", web::post().to(endpoints::auth::login_endpoint::response))
        .route("/api/auth/accesstoken", web::get().to(endpoints::auth::get_accesstoken_endpoint::response))
        .route("/api/auth/me", web::get().to(endpoints::auth::me_endpoint::response))
        // user API
        .route("/api/user-api/get_profile_picture", web::get().to(endpoints::user::profile_picture_endpoint::response))
        .route("/api/user-api/upload_post", web::post().to(endpoints::user::upload_new_image_endpoint::response))
        .route("/api/user-api/get_posts", web::get().to(endpoints::user::get_user_posts_endpoint::response))
        .route("/api/user-api/get_user_information", web::get().to(endpoints::user::get_user_information_endpoint::response))
        // post API
        .route("/api/post-api/get_post_image", web::get().to(endpoints::post::get_post_image::response))
        .route("/api/post-api/get_post_data", web::get().to(endpoints::post::get_post_data::response))
        .route("/api/post-api/like_post", web::post().to(endpoints::post::like_post::response))
        .route("/api/post-api/comment_post", web::post().to(endpoints::post::comment_post::response))
        // file server
        .service(Files::new("/dashboard", "./build").index_file("index.html"))
        .service(Files::new("/profile", "./build").index_file("index.html"))
        .service(Files::new("/user/{id}", "./build").index_file("index.html"))
        .service(Files::new("/login", "./build").index_file("index.html"))
        .service(Files::new("/register", "./build").index_file("index.html"))
        .service(Files::new("/", "./build").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
