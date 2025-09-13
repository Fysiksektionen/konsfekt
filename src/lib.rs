pub mod database;
pub mod auth;
pub mod types;

use sqlx::{Pool, Sqlite};

pub struct AppState {
    pub db: Pool<Sqlite>,
}
