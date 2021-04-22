use sqlx::{Pool, query_as};
use sqlx::MySql;
use sqlx::query;
use crate::database::models::user;
use std::time::{UNIX_EPOCH, SystemTime};


// This function stores a new user into the database
pub async fn register(user: &user::User, conn: &Pool<MySql>) -> bool {

    let unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let status = query!("INSERT INTO `user_accounts` (
    `user_id`,
    `displayname`,
    `email`,
    `password`,
    `num_follower`,
    `num_subscriptions`,
    `subscriptions`,
     `created_at`
     ) VALUES (NULL, ?, ?, ?, '0', '0', '', ?);",
     user.displayname,
     user.email,
     user.password,
     unix
    ).execute(conn).await;

    status.is_ok()
}



