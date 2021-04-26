use sqlx::{Pool, MySql, query, query_as};
use chrono::{NaiveDateTime, Utc};
use crate::database::models::user::User;
use crate::utils::random::generate_token;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::Ref;

/// The database model for the
/// refresh_token table.
/// It implements different functions you can run on this
/// database model
pub struct RefreshToken {
    pub id: i32,
    pub username: String,
    pub token: String,
    pub deadline: NaiveDateTime
}

/// implementation for printing the
/// RefreshToken struct
impl std::fmt::Display for RefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "----------------------\n{}\n{}\n{}\n{}\n----------------------", self.id, self.username, self.token, self.deadline)
    }
}

impl RefreshToken {

    fn new() -> RefreshToken {
        RefreshToken {
            id: 0,
            username: "".to_string(),
            token: "".to_string(),
            deadline: chrono::NaiveDateTime::from_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, 0)
        }
    }

    /// This function creates an new Refresh token
    /// and stores it into the database
    /// The refresh token is returned by the function
    pub async fn create_new(conn: &Pool<MySql>, usr: &User) -> RefreshToken {

        let token = generate_token();

        let mut unix = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        unix = unix + 432000;
        let deadline_unix: i64 = unix as i64;
        let deadline = chrono::NaiveDateTime::from_timestamp(deadline_unix, 0);

        query!("INSERT INTO `refresh_token` (`id`, `username`, `token`, `deadline`) VALUES (NULL, ?, ?, ?);",
            &usr.displayname, &token, &deadline
        ).execute(conn).await.unwrap();

        RefreshToken {
            id: 0,
            username: usr.displayname.clone(),
            token,
            deadline
        }
    }

    /// This functions checks, if a token exists and is valid.
    /// It returns the status and refresh token
    pub async fn check_existence(&self, conn: &Pool<MySql>) -> bool {

        let token: Vec<RefreshToken> = query_as!(RefreshToken, "SELECT * FROM `refresh_token` WHERE `username`=? AND `token`=? AND `deadline`>?",
            &self.username, &self.token, &self.deadline
        ).fetch_all(conn).await.unwrap();



        token.len() == 1
    }
}

