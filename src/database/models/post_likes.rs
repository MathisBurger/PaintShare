use serde::Serialize;
use sqlx::{query, MySql, Pool, query_as, Error};
use std::time::{SystemTime, UNIX_EPOCH};

/// The database model for the
/// post_likes table. It contains
/// all the information about an like
/// on a specific post.
#[derive(Serialize)]
pub struct PostLikes {
    pub like_id: i32,
    pub post_id: i32,
    pub owner: String,
    pub timestamp: i64
}

impl PostLikes {

    /// This function generates an
    /// empty instance of the PostLikes
    /// model and returns it.
    pub fn new() -> PostLikes {
        PostLikes {
            like_id: 0,
            post_id: 0,
            owner: "".to_string(),
            timestamp: 0
        }
    }

    /// This function inserts a new like into the
    /// user likes table with timestamp now
    /// as value for the timestamp
    pub async fn insert_like(conn: &Pool<MySql>, post_id: i32, owner: &String) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        query!("INSERT INTO `post_likes` (`like_id`, `post_id`, `owner`, `timestamp`) VALUES (NULL, ?, ?, ?);",
            post_id, owner, timestamp
        ).execute(conn).await.unwrap();
    }

    /// This function fetches all likes of an post by the
    /// post_id and returns them as a vector of
    /// the PostLikes model.
    pub async fn get_all_likes_of_post(conn: &Pool<MySql>, post_id: i32) -> Vec<PostLikes> {
        let likes: Vec<PostLikes> = query_as!(PostLikes, "SELECT * FROM `post_likes` WHERE `post_id`=?", post_id)
            .fetch_all(conn).await.unwrap();
        likes
    }

    /// This function checks if the given user
    /// liked the given post by fetching all
    /// likes of the post and checking every entry
    pub async fn user_liked_post(conn: &Pool<MySql>, post_id: i32, username: &String) -> bool {
        let like: Vec<PostLikes> = query_as!(PostLikes, "SELECT * FROM `post_likes` WHERE `post_id`=? AND `owner`=?",
            post_id, username
        ).fetch_all(conn).await.unwrap();
        like.len() == 1
    }
}