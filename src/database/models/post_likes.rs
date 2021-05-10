use serde::Serialize;
use sqlx::{query, MySql, Pool, query_as, Error};

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

    /// This function fetches all likes of an post by the
    /// post_id and returns them as a vector of
    /// the PostLikes model.
    pub async fn get_all_likes_of_post(conn: &Pool<MySql>, post_id: i32) -> Vec<PostLikes> {
        let likes: Vec<PostLikes> = query_as!(PostLikes, "SELECT * FROM `post_likes` WHERE `post_id`=?", post_id)
            .fetch_all(conn).await.unwrap();
        likes
    }
}