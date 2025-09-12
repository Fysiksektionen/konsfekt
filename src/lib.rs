pub mod database;

use sqlx::{Pool, Sqlite};

pub struct AppState {
    pub db: Pool<Sqlite>,
}
