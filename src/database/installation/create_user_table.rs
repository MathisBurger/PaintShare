use sqlx::Pool;
use sqlx::MySql;
use sqlx::query;

/// This function creates the user_accounts table required
/// to store the general account data about the
/// different users
pub async fn create_user_table(conn: &Pool<MySql>) {
    let resp = query!("CREATE TABLE `user_accounts` (
    `user_id` INT NOT NULL AUTO_INCREMENT,
    `displayname` TEXT NOT NULL,
    `email` TEXT NOT NULL,
    `password` TEXT NOT NULL,
    `num_follower` INT NOT NULL,
    `num_subscriptions` INT NOT NULL,
    `subscriptions` TEXT NOT NULL,
    `created_at` BIGINT NOT NULL,
    PRIMARY KEY(`user_id`)
    );").execute(conn).await;
    match resp {
        Ok(T) => println!("created user table"),
        Err(T) => println!("user table already exists")
    }
}