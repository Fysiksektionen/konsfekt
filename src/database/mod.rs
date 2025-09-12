pub mod crud;

use std::str::FromStr;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

pub async fn init_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_options = SqliteConnectOptions::from_str("sqlite://db/db.sqlite")?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options).await?;

    sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    return Ok(pool);
}
