pub mod database;
pub mod auth;
pub mod routes;
pub mod utils;

use std::{env, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use reqwest::Client;
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct EnvironmentVariables {
    pub is_debug: bool,
    pub static_frontend: bool,
    pub frontend_url: String,
    pub site_domain: String,
    pub google_client_id: String,
    pub google_client_secret: String,
}

// Prod (always 0.0.0.0)
// - caddy locally
// - caddy other
//
// Dev
// - local 
// - lan

impl EnvironmentVariables {
    pub fn new() -> Self {
        let _ = dotenv::dotenv();
        let mut static_frontend = env::var("STATIC_FRONTEND").unwrap_or("true".into()).parse::<bool>().unwrap_or(false);
        let is_debug = cfg!(debug_assertions);
        if !is_debug {
            static_frontend = true;
        }
        EnvironmentVariables {
            is_debug,
            static_frontend,
            frontend_url: match static_frontend { 
                true => String::from("/"),
                false => String::from("http://127.0.0.1:5173"),
            },
            site_domain: match is_debug {
                true => String::from("http://127.0.0.1:8080"),
                false => env::var("SITE_DOMAIN").unwrap(),
            },
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
        }
    }
}

pub struct AppState {
    pub db: Pool<Sqlite>,
    pub client: Client,
    pub env: EnvironmentVariables,
}

impl AppState {
    pub fn from(pool: Pool<Sqlite>, env_vars: EnvironmentVariables) -> Self {
        AppState {
            db: pool,
            client: reqwest::Client::new(),
            env: env_vars
        }
    }
}

#[derive(Debug)]
pub enum AppError {
    ClientError(reqwest::Error),
    DatabaseError(sqlx::Error),
    GenericError(String),

    SessionError(String),

    
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let (status, message) = match &self {
            Self::ClientError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("ClientError: {e}")),
            Self::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("DatabaseError: {e}")),
            Self::GenericError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("GenericError: {e}")),
            Self::SessionError(e) => (StatusCode:: INTERNAL_SERVER_ERROR, format!("SessionError: {e}"))
        };

        HttpResponse::build(status).body(message)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ClientError(err) => write!(f, "Client error: {}", err),
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::GenericError(err) => write!(f, "Generic error: {}", err),
            AppError::SessionError(err) => write!(f, "Session error: {}", err),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ClientError(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl std::error::Error for AppError {}
