use sqlx::{Pool, MySql, query, query_as};
use crate::utils::hashing;


// The database model for
// the user accounts table.
// It implements different functions you can run on this
// database model
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

    // This function returns an empty
    // user, without storing it into the
    // database.
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

    // This function checks if there already
    // exists a user in the database identified
    // by a unique username (displayname)
    pub async fn check_user_existance(&self, conn: &Pool<MySql>) -> bool {

        let user: Vec<User> = query_as!(User, "SELECT * FROM `user_accounts` WHERE `displayname`=?", self.displayname)
            .fetch_all(conn).await.unwrap();

        user.len() == 1
    }

    // This function checks if the given
    // login creds are valid and the
    // user is allowed to login
    pub async fn check_login(&self, conn: &Pool<MySql>) -> bool {

        let user: User = query_as!(User, "SELECT * FROM `user_accounts` WHERE `displayname`=?", self.displayname)
            .fetch_one(conn).await.unwrap();

        hashing::verify(&user.password, &self.password)
    }
}