use sqlx::Pool;
use sqlx::MySql;
use sqlx::query;

/// This function creates the post_likes table required
/// to store all posts of every user
pub async fn create_post_like_table(conn: &Pool<MySql>) {
    let resp = query!("CREATE TABLE `post_likes` (
    `like_id` INT NOT NULL AUTO_INCREMENT,
    `post_id` INT NOT NULL,
    `owner` TEXT NOT NULL,
    `timestamp` BIGINT NOT NULL,
    PRIMARY KEY (`like_id`)
);").execute(conn).await;
    match resp {
        Ok(T) => println!("created user post likes table"),
        Err(T) => println!("user post likes table already exists")
    }
}