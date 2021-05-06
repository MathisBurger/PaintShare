use crate::database::models::user::User;
use actix_web::web;
use std::collections::BTreeMap;
use serde::Deserialize;
use sqlx::{Pool, MySql};

#[derive(Deserialize)]
pub struct Query {
    pub user: Option<String>
}


/// This function checks if an username parameter is
/// given and returns the matching database model
/// if the user is not given in the params, it returns
/// the database model of the session-owner
pub async fn get_user_from_request(query: &web::Query<Query>, verification: &(bool, BTreeMap<String, String>), conn: &Pool<MySql>) -> (bool, User) {

    let user_itself = match &query.user {
        Some(T) => false,
        None => true
    };

    let mut user = User::new();

    if user_itself {
        let user_id = verification.1.get("user_id").unwrap().parse().unwrap();
        user = User::new().get_user_by_id(user_id, conn).await.unwrap();
    } else {
        let username = query.user.as_ref().unwrap();
        let usr  = User::new()
            .get_user_by_username(username, conn).await;

        let exists = match &usr {
            Ok(T) => true,
            Err(E) => false
        };

        if !exists {
            return (false, User::new());
        }

        user = usr.unwrap();
    }
    return (true, user);
}