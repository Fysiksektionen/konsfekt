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
    pub swish_number: String,
    pub use_swish_sandbox: bool,
    pub swish_api_url: String,
}

fn required_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Missing required environment variable: {name}"))
}

impl EnvironmentVariables {

    pub fn from_args(args: args::Args) -> Self {
        let _ = dotenv::dotenv();

        let is_debug = cfg!(debug_assertions);
        let static_frontend = !args.run_locally || args.static_frontend;

        let swish_environment = required_env("SWISH_ENVIRONMENT");
        let use_swish_sandbox = match swish_environment.as_str() {
            "prod" => false, "sandbox" => true, 
            _ => panic!("SWISH_ENVIRONMENT can only take values 'sandbox' and 'prod'")
        };

        EnvironmentVariables {
            is_debug,
            static_frontend,
            frontend_url: match static_frontend { 
                // If not static frontend, serve from default vite port
                true => String::from("/"),
                false => String::from("http://127.0.0.1:5173"),
            },
            site_domain: match args.run_locally {
                true => String::from("http://127.0.0.1:8080"),
                false => required_env("SITE_DOMAIN"),
            },
            google_client_id: required_env("GOOGLE_CLIENT_ID"),
            google_client_secret: required_env("GOOGLE_CLIENT_SECRET"),
            swish_number: required_env("SWISH_NUMBER"),
            use_swish_sandbox,
            swish_api_url: match use_swish_sandbox {
                true => String::from("https://staging.getswish.pub.tds.tieto.com/swish-cpcapi/api/v2/paymentrequests/"),
                false => String::from("https://cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/"),
            }
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
    pub fn new() -> Self {
        let json_str = fs::read_to_string(String::from("./permission_table.json")).expect("Could not open permission table file");
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
        if !env_vars.use_swish_sandbox {
            unimplemented!("Need to figure out production certificates");
        }

        let cert_bytes = fs::read("certificates/sandbox/myCertificate.p12").unwrap();
        let ca_cert = fs::read("certificates/sandbox/myCertificate.pem").unwrap();

        let identity = Identity::from_pkcs12_der(&cert_bytes, "swish").unwrap();
        let ca = Certificate::from_pem(&ca_cert).unwrap();

        AppState {
            db: pool,
            client: reqwest::Client::builder()
                .identity(identity)
                .add_root_certificate(ca)
                .build()
                .expect("Could not build reqwest::Client"),
            env: env_vars.clone(),
            permission_table: PermissionTable::new()
        }
    }
}
