use sqlx::{Pool, MySql, query, query_as, Error};
use crate::utils::hashing;
use crate::utils::iterator::check_duplicates;
use std::borrow::Borrow;
use std::fmt::Debug;


/// The database model for
/// the user accounts table.
/// It implements different functions you can run on this
/// database model
#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub displayname: String,
    pub email: String,
    pub password: String,
    pub num_follower: i32,
    pub num_subscriptions: i32,
    pub subscriptions: String,
    pub profile_picture: String,
    pub created_at: i64
}

impl User {

    /// This function returns an empty
    /// user, without storing it into the
    /// database.
    pub fn new() -> User {
       let mut usr = User {
           user_id: 0,
           displayname: "".to_string(),
           email: "".to_string(),
           password: "".to_string(),
           num_follower: 0,
           num_subscriptions: 0,
           subscriptions: "".to_string(),
           profile_picture: "".to_string(),
           created_at: 0
       };
        return usr;
    }

    /// This function checks if there already
    /// exists a user in the database identified
    /// by a unique username (displayname)
    pub async fn check_user_existance(&self, conn: &Pool<MySql>) -> bool {
        let user: Vec<User> = query_as!(User, "SELECT * FROM `user_accounts` WHERE `displayname`=?", self.displayname)
            .fetch_all(conn).await.unwrap();
        user.len() == 1
    }

    /// This function checks if the given
    /// login creds are valid and the
    /// user is allowed to login
    pub async fn check_login(&self, conn: &Pool<MySql>) -> bool {
        let user: User = query_as!(User, "SELECT * FROM `user_accounts` WHERE `displayname`=?", self.displayname)
            .fetch_one(conn).await.unwrap();

        hashing::verify(&user.password, &self.password)
    }

    /// This function returns the user
    /// identified by the given username
    /// from the database
    pub async fn get_user_by_username(&self, username: &String, conn: &Pool<MySql>) -> Result<User, Error> {
        query_as!(User, "SELECT * FROM `user_accounts` WHERE `displayname`=?", username)
            .fetch_one(conn).await
    }

    /// This function returns the user
    /// identified by the given user_id
    /// from the database
    pub async fn get_user_by_id(&self, id: i32, conn: &Pool<MySql>) -> Result<User, Error> {
        query_as!(User, "SELECT * FROM `user_accounts` WHERE `user_id`=?", id)
            .fetch_one(conn).await
    }

    /// This function tries to add a new subscription to the subscription
    /// string of the base_user. It makes multiple validation checks
    /// and returns the status of the action as a boolean value
    pub async fn add_subscription_to_user(conn: &Pool<MySql>, base_user: i32, to_subscribe: i32) -> bool {
        let mut user = match User::new().get_user_by_id(base_user, conn).await {
            Ok(t) => (true, t),
            Err(e) => (false, User::new())
        };
        if !user.0 {
            return false
        }
        if check_duplicates(&user.1.subscriptions, to_subscribe) {
            return false
        }
        let mut new_subs = String::new();
        if &user.1.subscriptions == &String::new() {
            new_subs = to_subscribe.to_string();
        } else {
            new_subs = user.1.subscriptions + " " + &*to_subscribe.to_string();
        }
        query!("UPDATE `user_accounts` SET `subscriptions`=? WHERE `user_id`=?", new_subs, base_user)
            .execute(conn).await.is_ok()
    }

    /// This function tries to remove a subscription from
    /// the given user. If the user has not subscribed
    /// the other user, the function returns false.
    /// Otherwise it returns the state of the action
    pub async fn remove_subscription_from_user(conn: &Pool<MySql>, base_user: i32, to_remove: i32) -> bool {
        let mut user = match User::new().get_user_by_id(base_user, conn).await {
            Ok(t) => (true, t),
            Err(e) => (false, User::new())
        };
        if !user.0 {
            return false
        }
        if !check_duplicates(&user.1.subscriptions, to_remove) {
            return false
        }
        let v = &user.1.subscriptions.split(" ").collect::<Vec<&str>>();
        let mut new_subs = String::new();
        for i in 0..v.len() {
            if &v[i].trim().parse::<i32>().unwrap() != &to_remove {
                if i != 0 {
                    new_subs.push_str(" ");
                }
                new_subs.push_str(v[i].trim());
            }
        }
        query!("UPDATE `user_accounts` SET `subscriptions`=? WHERE `user_id`=?", new_subs, base_user)
            .execute(conn).await.is_ok()
    }

    /// This function increases the number of follower
    /// of the given user by the given value
    /// without checking user existance
    pub async fn increase_follower_of_user(conn: &Pool<MySql>, user_id: i32, by: i32) {
        let new_follower = User::new()
            .get_user_by_id(user_id, conn).await.unwrap()
            .num_follower + by;
        query!("UPDATE `user_accounts` SET `num_follower`=? WHERE `user_id`=?", new_follower, user_id)
            .execute(conn).await.unwrap();
    }

    /// This function increases the number of subscriptions
    /// of the given user by the given value
    /// without checking user existance
    pub async fn increase_subscriptions_of_user(conn: &Pool<MySql>, user_id: i32, by: i32) {
        let new_subs = User::new()
            .get_user_by_id(user_id, conn).await.unwrap()
            .num_follower + by;
        query!("UPDATE `user_accounts` SET `num_subscriptions`=? WHERE `user_id`=?", new_subs, user_id)
            .execute(conn).await.unwrap();
    }
}