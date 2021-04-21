use sqlx::Pool;
use sqlx::MySql;
use sqlx::query;
use sqlx::query_as;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub displayname: String,
    pub email: String,
    pub password: String,
    pub num_follower: i32,
    pub num_subscriptions: i32,
    pub subscriptions: String,
    pub created_at: i64
}

impl User {

    pub fn new() -> User {

       let mut usr = User {
           user_id: 0,
           displayname: "".to_string(),
           email: "".to_string(),
           password: "".to_string(),
           num_follower: 0,
           num_subscriptions: 0,
           subscriptions: "".to_string(),
           created_at: 0
       };

        return usr;
    }

    pub async fn check_user_existance(&self, conn: &Pool<MySql>) -> bool {

        let user: Vec<User> = query_as!(User, "SELECT * FROM `user_accounts` WHERE `email`=?", self.email)
            .fetch_all(conn).await.unwrap();
        user.len() == 1
    }
}