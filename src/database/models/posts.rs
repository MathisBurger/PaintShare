use std::time::{SystemTime, UNIX_EPOCH};
use sqlx::{query, MySql, Pool, query_as, Error};
use crate::utils::storage::generate_post_path;
use crate::database::models::user::User;


/// The database model for the
/// posts table. It implements
/// all the functionality given
/// by the post model.
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub storage_destination: String,
    pub comment: String,
    pub tags: String,
    pub likes: i32,
    pub comments: i32,
    pub created_at: i64
}


impl Post {

    /// This function returns an new, empty instance
    /// of the post model, but don`t
    /// saves it to the database.
    pub fn new() -> Post {
        Post {
            id: 0,
            user_id: 0,
            storage_destination: "".to_string(),
            comment: "".to_string(),
            tags: "".to_string(),
            likes: 0,
            comments: 0,
            created_at: 0
        }
    }

    /// This function inserts a new post
    /// into the database and returns the
    /// database model as instance of the
    /// post struct
    pub async fn create_new(
        conn: &Pool<MySql>,
        user_id: i32,
        comment: &String,
        tags: &String,
        storage_destination: String
    ) -> Post {

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        query!("INSERT INTO `user_posts` (`id`, `user_id`, `storage_destination`, `comment`, `tags`, `likes`, `comments`, `created_at`) VALUES (NULL, ?, ?, ?, ?, '0', '0', ?);",
            user_id, &storage_destination, comment, tags, &timestamp
        ).execute(conn).await.unwrap();
        Post {
            id: 0,
            user_id,
            storage_destination,
            comment: "".to_string(),
            tags: "".to_string(),
            likes: 0,
            comments: 0,
            created_at: timestamp
        }
    }

    /// This function returns the data of all
    /// posts a user has posted on his
    /// profile as Vec<Post>
    pub async fn get_all_posts_of_user(conn: &Pool<MySql>, user_id: i32) -> Vec<Post> {
        let posts: Vec<Post> = query_as!(Post, "SELECT * FROM `user_posts` WHERE `user_id`=?", user_id)
            .fetch_all(conn).await.unwrap();
        posts
    }

    /// This function queries a specific post by it`s id and returns it
    /// as the post database model
    pub async fn get_post_by_id(conn: &Pool<MySql>, post_id: i32) -> Result<Post, Error> {
        query_as!(Post, "SELECT * FROM `user_posts` WHERE `id`=?", post_id)
            .fetch_one(conn).await
    }
}