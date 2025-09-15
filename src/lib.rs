pub mod database;
pub mod auth;
pub mod types;

use std::{env, error::Error};

use reqwest::Client;
use sqlx::{Pool, Sqlite};

pub struct EnvironmentVariables {
    pub bankid_api: String,
    pub frontend_url: String,
    pub hmac_secret: String,
}

impl EnvironmentVariables {
    pub fn new() -> Self {
        EnvironmentVariables { 
            bankid_api: env::var("BANKID_API").unwrap(), 
            frontend_url: env::var("FRONTEND_URL").unwrap(), 
            hmac_secret: env::var("HMAC_SECRET").unwrap() 
        }
    }
}

pub struct AppState {
    pub db: Pool<Sqlite>,
    pub client: Client,
    pub env_vars: EnvironmentVariables,
}

impl AppState {
    pub fn from(pool: Pool<Sqlite>) -> Self {
        AppState {
            db: pool,
            client: reqwest::Client::new(),
            env_vars: EnvironmentVariables::new()

        }
    }
}

pub enum AppError {
    ClientError(reqwest::Error),
    DatabaseError(sqlx::Error)

}
