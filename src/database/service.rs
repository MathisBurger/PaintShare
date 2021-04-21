use sqlx::Pool;
use sqlx::MySql;

use crate::database::installation::{create_user_table, create_token_table};

pub async fn init_tables(conn: &Pool<MySql>) {

    create_user_table::create_user_table(&conn).await;
    create_token_table::create_refresh_token_table(&conn).await;
}
