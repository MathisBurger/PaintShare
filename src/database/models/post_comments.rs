

/// The database model for the
/// post_comments table. It
/// collects all the information
/// about a comment of a specific post
pub struct PostComment {
    pub comment_id: i32,
    pub post_id: i32,
    pub user_id: i32,
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
            user_id: 0,
            message: "".to_string()
        }
    }
}