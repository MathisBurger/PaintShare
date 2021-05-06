use std::time::{SystemTime, UNIX_EPOCH};
use sqlx::{query, MySql, Pool};
use crate::utils::storage::generate_post_path;


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
        query!("INSERT INTO `user_posts` (`id`, `user_id`, `storage_destination`, `comment`, `tags`, `created_at`) VALUES (NULL, ?, ?, ?, ?, ?);",
            user_id, &storage_destination, comment, tags, &timestamp
        ).execute(conn).await.unwrap();
        Post {
            id: 0,
            user_id,
            storage_destination,
            comment: "".to_string(),
            tags: "".to_string(),
            created_at: timestamp
        }
    }
}