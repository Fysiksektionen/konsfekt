//! The konsfekt binary can a number or arguments.
//! All are defined here.
//! Use `--local` if you want to run the code locally (not use the env variable `SITE_DOMAIN`)
//! This will also only run the backend (not serve the frontend) so you'll need to run
//! the frontend separately.
//! If you want it to serve the frontend, additionally pass `--static`.
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long = "local")]
    pub run_locally: bool,
    #[arg(long = "static")]
    pub static_frontend: bool
}
