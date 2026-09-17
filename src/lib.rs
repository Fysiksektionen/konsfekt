//! Library crate for konsfekt: environment/configuration loading, the shared
//! [`AppState`] passed to every request handler, and the [`Role`] permission model.
//! The HTTP routes live in [`routes`], persistence in [`database`], authentication
//! in [`auth`], and shared response/DTO types in [`model`].

pub mod database;
pub mod auth;
pub mod routes;
pub mod utils;
pub mod model;
pub mod error;
pub mod args;

use std::{env, fs};

use reqwest::{Certificate, Client, Identity};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use time::macros::format_description;

/// Configuration resolved once at startup from CLI [`args::Args`] and environment
/// variables (loaded via `.env` through [`dotenv::dotenv`]).
#[derive(Clone)]
pub struct EnvironmentVariables {
    /// True for debug builds (`cfg!(debug_assertions)`), used to enable extra logging.
    pub is_debug: bool,
    /// True when the backend should serve the built frontend as static files,
    /// rather than expecting a separately running frontend dev server.
    pub static_frontend: bool,
    /// Base URL the frontend is reachable at; `"/"` when serving statically,
    /// otherwise the Vite dev server address, used for CORS and redirects.
    pub frontend_url: String,
    /// True if `site_domain` uses the `https` scheme; used to decide whether
    /// cookies are marked `Secure`.
    pub is_running_https: bool,
    /// Public URL of the site, taken from `SITE_DOMAIN`, or `http://127.0.0.1:8080`
    /// when running with `--local`.
    pub site_domain: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    /// The Swish merchant number payments are requested against.
    pub swish_number: String,
    /// True to talk to the Swish sandbox API instead of production, set via `SWISH_ENVIRONMENT`.
    pub use_swish_sandbox: bool,
    /// Base URL of the Swish payment request API, chosen based on `use_swish_sandbox`.
    pub swish_api_url: String,
    /// Randomly generated at startup, used to sign/verify the tokens that let a
    /// purchase be undone. Not persisted, so undo tokens stop working across restarts.
    pub undo_purchase_secret: String,
    /// Whether the periodic database backup task ([`main`](crate) sets this up) is enabled.
    pub backups_enabled: bool,
    /// List of times of day at which database backups runs, from `BACKUP_SCHEDULE` ("hh:mm").
    pub backup_schedule: Vec<time::Time>
}

/// Reads an environment variable, panicking with a descriptive message if it is unset.
fn required_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Missing required environment variable: {name}"))
}

impl EnvironmentVariables {

    /// Builds [`EnvironmentVariables`] from parsed CLI arguments and the process
    /// environment (loading a `.env` file first, if present).
    ///
    /// # Panics
    /// Panics if a required environment variable is missing, if `SWISH_ENVIRONMENT`
    /// is not `"prod"` or `"sandbox"`, or if `BACKUP_INTERVAL` cannot be parsed as `hh:mm:ss`.
    pub fn from_args(args: args::Args) -> Self {
        let _ = dotenv::dotenv();

        let is_debug = cfg!(debug_assertions);
        let static_frontend = !args.run_locally || args.static_frontend;

        let swish_environment = required_env("SWISH_ENVIRONMENT");
        let use_swish_sandbox = match swish_environment.as_str() {
            "prod" => false, "sandbox" => true, 
            _ => panic!("SWISH_ENVIRONMENT can only take values 'sandbox' and 'prod'")
        };

        let site_domain = required_env("SITE_DOMAIN");
        EnvironmentVariables {
            is_debug,
            static_frontend,
            frontend_url: match static_frontend { 
                // If not static frontend, serve from default vite port
                true => String::from("/"),
                false => String::from("http://127.0.0.1:5173"),
            },
            is_running_https: site_domain.starts_with("https"),
            site_domain: match args.run_locally {
                true => String::from("http://127.0.0.1:8080"),
                false => site_domain,
            },
            google_client_id: required_env("GOOGLE_CLIENT_ID"),
            google_client_secret: required_env("GOOGLE_CLIENT_SECRET"),
            swish_number: required_env("SWISH_NUMBER"),
            use_swish_sandbox,
            swish_api_url: match use_swish_sandbox {
                true => String::from("https://staging.getswish.pub.tds.tieto.com/swish-cpcapi/api/v2/paymentrequests/"),
                false => String::from("https://cpc.getswish.net/swish-cpcapi/api/v2/paymentrequests/"),
            },
            undo_purchase_secret: utils::gen_secure_random_str().expect("Could not generate secret for undoable purchases."),
            backups_enabled: match required_env("ENABLE_BACKUPS").as_str() {
                "true" | "True" | "t"  => true,
                _ => false
            },

            backup_schedule: required_env("BACKUP_SCHEDULE")
                .as_str()
                .split(",")
                .skip_while(|x| x.is_empty())
                .map(|x|
                    time::Time::parse(
                        x.trim_start().trim_end(),
                        &format_description!("[hour]:[minute]")
                    ).expect(format!("Unable to parse BACKUP_SCHEDULE timestamp, use format \"hh:mm\". Got value {}", x).as_str()),
                ).collect()
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
    /// Parses a role name (case-insensitive) into a [`Role`], defaulting to
    /// [`Role::User`] for any unrecognized string.
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

/// Shared application state handed to every request handler via actix-web's `Data`.
pub struct AppState {
    /// The SQLite connection pool.
    pub db: Pool<Sqlite>,
    /// HTTP client configured with the Swish client certificate, used to call the Swish API.
    pub client: Client,
    pub env: EnvironmentVariables,
}

impl AppState {
    /// Builds [`AppState`], including a [`reqwest::Client`] configured with the
    /// Swish sandbox client certificate and CA for mutual TLS.
    ///
    /// # Panics
    /// Panics if `env_vars.use_swish_sandbox` is false (production certificates are
    /// not yet implemented), or if the sandbox certificate files under
    /// `certificates/sandbox/` are missing or invalid.
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
            env: env_vars.clone()
        }
    }
}
