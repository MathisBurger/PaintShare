use sqlx::Pool;
use sqlx::MySql;
use sqlx::query;

/// This function creates the post_comments table required
/// to store all posts of every user
pub async fn create_post_comments_table(conn: &Pool<MySql>) {
    let resp = query!("CREATE TABLE `post_comments` (
    `comment_id` INT NOT NULL AUTO_INCREMENT,
    `post_id` INT NOT NULL,
    `owner` TEXT NOT NULL,
    `message` TEXT NOT NULL,
    PRIMARY KEY (`comment_id`)
);").execute(conn).await;
    match resp {
        Ok(T) => println!("created user post comments table"),
        Err(T) => println!("user post comments table already exists")
    }
}