pub mod crud;
pub mod model;

use std::{fs, path::Path, str::FromStr};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

pub async fn init_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let _ = fs::create_dir_all(Path::new("./db/uploads/images/product"));

    let db_options = SqliteConnectOptions::from_str("sqlite://db/db.sqlite")?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options).await?;

    sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    return Ok(pool);
}
