use actix_web::{HttpServer, App, web, middleware};
use dotenv::dotenv;
use sqlx::{mysql, Pool, MySql, migrate};
use actix_cors::Cors;

mod database;
mod utils;
mod endpoints;

pub struct ServerData {
    pub db: Pool<MySql>
}

#[actix_web::main]
async fn main() {

    // init .env handling
    dotenv().ok();

    let conn = mysql::MySqlPool::connect(&utils::enviroment_handler::load_param("DATABASE_URL"))
    .await.expect("Cannot create database connection");

    migrate!("./migrations/")
        .run(&conn)
        .await
        .ecpect("database migration failed")

    HttpServer::new(move || {
        App::new()
        .data(ServerData {db: conn.clone()})
        .wrap(middleware::Logger::default())
        .wrap(Cors::new().supports_credentials().finish())
        .route("/api", web::get().to(endpoints::default_endpoint::response))
    
    })
    .bind("0.0.0.0:8080")
    .expect(&format!("couldn't bind to address {}", "0.0.0.0:8080"))
    .run()
    .await
    .expect("couldn't run server");

}
