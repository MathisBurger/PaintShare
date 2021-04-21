extern crate pretty_env_logger;
#[macro_use] extern crate log;

use actix_web::{HttpServer, App, web, middleware};
use dotenv::dotenv;
use sqlx::{mysql, Pool, MySql};
use actix_cors::Cors;

mod database;
mod utils;
mod endpoints;

pub struct ServerData {
    pub db: Pool<MySql>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    pretty_env_logger::init();

    // init .env handling
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
        .wrap(middleware::Logger::default())
        .wrap(Cors::new().supports_credentials().finish())
        .route("/api", web::get().to(endpoints::default_endpoint::response))
        .route("/api/user/register", web::post().to(endpoints::register_endpoint::response))
    
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
