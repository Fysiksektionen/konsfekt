pub mod database;
pub mod auth;
pub mod routes;
pub mod utils;
pub mod model;
pub mod error;
pub mod args;

use std::{collections::HashMap, env, fs};

use reqwest::{Certificate, Client, Identity};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

#[derive(Clone)]
pub struct EnvironmentVariables {
    pub is_debug: bool,
    pub static_frontend: bool,
    pub frontend_url: String,
    pub site_domain: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub permission_table_path: String,
}

impl EnvironmentVariables {

    pub fn from_args(args: args::Args) -> Self {
        let _ = dotenv::dotenv();

        let is_debug = cfg!(debug_assertions);
        let static_frontend = args.mode == args::Mode::Tunnel || args.mode == args::Mode::Prod || args.static_frontend;

        EnvironmentVariables {
            is_debug,
            static_frontend,
            frontend_url: match static_frontend { 
                // If not static frontend, serve from default vite port
                true => String::from("/"),
                false => String::from("http://127.0.0.1:5173"),
            },
            site_domain: match args.mode {
                args::Mode::Local => String::from("http://127.0.0.1:8080"),
                args::Mode::Prod => env::var("PRODUCTION_DOMAIN").unwrap(),
                args::Mode::Tunnel => env::var("TUNNEL_DOMAIN").unwrap(),
            },
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap(),
            permission_table_path: env::var("PERMISSION_TABLE_PATH").unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "role", rename_all = "lowercase")]
/// Discriminants: permission levels
pub enum Role {
    User = 0,
    Bot = 1,
    Maintainer = 2,
    Admin = 3,
}

impl Role {
    pub fn from_str(string: &str) -> Role {
        match string.to_lowercase().as_str() {
            "user" => Role::User,
            "bot" => Role::Bot,
            "maintainer" => Role::Maintainer,
            "admin" => Role::Admin,
            _ => Role::User
        }
    }
}

#[derive(Clone)]
pub struct PermissionTable {
    table: HashMap<String, Role>,
}

impl PermissionTable {
    pub fn from(file_path: &str) -> Self {
        // We need permissions
        let json_str = fs::read_to_string(&file_path).expect(format!("Could not find file {}", file_path).as_str());
        let json: HashMap<String, Role> = serde_json::from_str(&json_str).unwrap();
        return PermissionTable { table: json };
    }

    pub fn empty() -> Self {
        // should log warning here
        PermissionTable { table: HashMap::new() }
    }

    pub fn get(&self, path: &str) -> Option<Role> {
        self.table.get(path).cloned()
    }

    pub fn check_access(&self, path: &str, user_perm: Role) -> bool {
        match self.get(path) {
            Some(perm) => user_perm >= perm, // greater than or equal permission level
            None => true // assume true if not in table
        }
    }

    pub fn contains(&self, path: &str) -> bool {
        self.table.contains_key(path)
    }
}

pub struct AppState {
    pub db: Pool<Sqlite>,
    pub client: Client,
    pub env: EnvironmentVariables,
    pub permission_table: PermissionTable,
}

impl AppState {
    pub fn from(pool: Pool<Sqlite>, env_vars: EnvironmentVariables) -> Self {

        let cert_bytes = fs::read("certificates/Swish_Merchant_TestCertificate_1234679304.p12").unwrap();
        let identity = Identity::from_pkcs12_der(&cert_bytes, "swish").unwrap();

        let ca_cert = fs::read("certificates/Swish_TLS_RootCA.pem").unwrap();
        let ca = Certificate::from_pem(&ca_cert).unwrap();

        AppState {
            db: pool,
            client: reqwest::Client::builder()
                .identity(identity)
                .add_root_certificate(ca)
                .build()
                .expect("Could not build reqwest::Client"),
            env: env_vars.clone(),
            permission_table: PermissionTable::from(&env_vars.permission_table_path)
        }
    }
}
