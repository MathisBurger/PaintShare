use sqlx::Pool;
use sqlx::MySql;
use sqlx::query;

/// This function creates the user_post table required
/// to store all posts of every user
pub async fn create_user_posts_table(conn: &Pool<MySql>) {
    let resp = query!("CREATE TABLE `user_posts` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `storage_destination` TEXT NOT NULL,
    `comment` TEXT NOT NULL,
    `tags` TEXT NOT NULL,
    `likes` INT NOT NULL,
    `comments` INT NOT NULL,
    `created_at` BIGINT NOT NULL,
    PRIMARY KEY (`id`)
);").execute(conn).await;
    match resp {
        Ok(T) => println!("created user posts table"),
        Err(T) => println!("user posts already exists")
    }
}