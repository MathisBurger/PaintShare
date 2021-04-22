use sqlx::Pool;
use sqlx::MySql;
use sqlx::query;

pub async fn create_refresh_token_table(conn: &Pool<MySql>) {
    let resp = query!("CREATE TABLE `refresh_token` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `username` TEXT NOT NULL , `token` TEXT NOT NULL,
    `deadline` DATETIME NOT NULL,
    PRIMARY KEY (`ID`)
    );").execute(conn).await;
    match resp {
        Ok(T) => println!("created refresh token table"),
        Err(T) => println!("refresh token table already exists")
    }
}