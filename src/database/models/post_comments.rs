use serde::{Serialize};
use sqlx::{query, MySql, Pool, query_as, Error};

/// The database model for the
/// post_comments table. It
/// collects all the information
/// about a comment of a specific post
#[derive(Serialize)]
pub struct PostComment {
    pub comment_id: i32,
    pub post_id: i32,
    pub owner: String,
    pub message: String
}

impl PostComment {

    /// This function generates an
    /// empty model of the PostComment
    /// type and returns it.
    pub fn new() -> PostComment {
        PostComment {
            comment_id: 0,
            post_id: 0,
            owner: "".to_string(),
            message: "".to_string()
        }
    }

    /// This function returns all comments of
    /// an specific post as a Vector of the
    /// PostComment model
    pub async fn get_all_comments_of_post(conn: &Pool<MySql>, post_id: i32) -> Vec<PostComment> {
        let posts: Vec<PostComment> = query_as!(PostComment, "SELECT * FROM `post_comments` WHERE `post_id`=?", post_id)
            .fetch_all(conn).await.unwrap();
        posts
    }
}