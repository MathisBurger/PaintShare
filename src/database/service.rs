use sqlx::Pool;
use sqlx::MySql;

use crate::database::installation::create_post_table::create_user_posts_table;
use crate::database::installation::create_post_like::create_post_like_table;
use crate::database::installation::create_user_table::create_user_table;
use crate::database::installation::create_token_table::create_refresh_token_table;
use crate::database::installation::create_post_comments_table::create_post_comments_table;


/// This function is called on server init
/// It creates the database tables, required
/// for the backend
pub async fn init_tables(conn: &Pool<MySql>) {

    create_user_table(&conn).await;
    create_refresh_token_table(&conn).await;
    create_user_posts_table(&conn).await;
    create_post_like_table(&conn).await;
    create_post_comments_table(&conn).await;
}
