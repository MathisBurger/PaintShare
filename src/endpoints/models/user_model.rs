use serde::Serialize;
use crate::database::models::user::User;

#[derive(Serialize)]
pub struct UserModel {
    pub user_id: i32,
    pub displayname: String,
    pub num_follower: i32,
    pub num_subscriptions: i32,
}

impl UserModel {

    /// This function generates a public model
    /// of the internal database model. It contains
    /// the data of the database model, but private
    /// data has been removed
    pub fn parse_model(user: User) -> UserModel {
        UserModel {
            user_id: user.user_id,
            displayname: user.displayname,
            num_follower: user.num_follower,
            num_subscriptions: user.num_subscriptions
        }
    }
}