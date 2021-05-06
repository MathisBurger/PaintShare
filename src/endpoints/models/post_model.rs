use serde::Serialize;
use crate::database::models::posts::Post;

#[derive(Serialize)]
pub struct PostModel {
    pub owner_id: i32,
    pub id: i32,
    pub comment: String,
    pub created_at: i64
}

impl PostModel {

    /// This function parses the internal
    /// database model to a publishable
    /// model without any secret information
    pub fn parse_model(post: Post) -> PostModel {
        PostModel {
            owner_id: post.user_id,
            id: post.id,
            comment: post.comment,
            created_at: post.created_at
        }
    }

    /// This function uses the parse_model function
    /// to convert an vector of database models to
    /// the output post model
    pub fn parse_models(posts: Vec<Post>) -> Vec<PostModel> {
        let mut new: Vec<PostModel> = vec![];
        for x in posts {
            new.push(self::PostModel::parse_model(x));
        }
        new
    }
}