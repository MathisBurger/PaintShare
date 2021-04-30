extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix_web::{HttpServer, App, web, middleware as actix_middleware};
use dotenv::dotenv;
use sqlx::{mysql, Pool, MySql};
use actix_cors::Cors;

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


    //database::service::init_tables(&conn).await;


    HttpServer::new(move || {
        App::new()
        .data(ServerData {db: conn.clone()})
        .wrap(actix_middleware::Logger::default())
        .wrap(Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
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
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
