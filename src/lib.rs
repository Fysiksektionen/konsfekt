pub mod database;
pub mod auth;
pub mod routes;
pub mod auth_redirect;

use std::{env, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use reqwest::Client;
use sqlx::{Pool, Sqlite};

pub struct EnvironmentVariables {
    pub site_domain: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
}

impl EnvironmentVariables {
    pub fn new() -> Self {
        EnvironmentVariables { 
            site_domain: env::var("SITE_DOMAIN").unwrap(), 
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI").unwrap(),
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

#[derive(Debug)]
pub enum AppError {
    ClientError(reqwest::Error),
    DatabaseError(sqlx::Error),
    GenericError(String)
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let (status, message) = match &self {
            Self::ClientError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("ClientError: {e}")),
            Self::DatabaseError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("DatabaseError: {e}")),
            Self::GenericError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("GenericError: {e}"))
        };
        HttpResponse::build(status).body(message)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ClientError(err) => write!(f, "Client error: {}", err),
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::GenericError(err) => write!(f, "Generic error: {}", err)
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
