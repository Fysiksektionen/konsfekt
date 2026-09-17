//! The konsfekt binary can a number or arguments.
//! All are defined here.
//! Use `--local` if you want to run the code locally (not use the env variable `SITE_DOMAIN`)
//! This will also only run the backend (not serve the frontend) so you'll need to run
//! the frontend separately.
//! If you want it to serve the frontend, additionally pass `--static`.
use clap::Parser;

/// Command-line arguments accepted by the konsfekt binary.
#[derive(Parser)]
pub struct Args {
    /// Run against `http://127.0.0.1:8080` instead of the `SITE_DOMAIN` env variable,
    /// and skip serving the frontend (run it separately).
    #[arg(long = "local")]
    pub run_locally: bool,
    /// Serve the built frontend as static files instead of proxying to the dev server.
    /// Only meaningful together with `--local`; without `--local` the frontend is always served statically.
    #[arg(long = "static")]
    pub static_frontend: bool
}
