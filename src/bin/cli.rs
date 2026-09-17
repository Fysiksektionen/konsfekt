//! A companion CLI for konsfekt, intended to authenticate against a running
//! server and manage the resulting session cookie. Not yet implemented — see
//! the TODO in [`main`].

use clap::{Parser, Subcommand};

/// Command-line arguments for the konsfekt CLI.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

/// Subcommands supported by the CLI.
#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Login, Logout etc (not yet implemented).
    Auth,
}

/// Entry point. Not yet implemented.
#[tokio::main]
async fn main() {
    // Todo använd reqwest för att logga in på backend och spara cookies
    // använd open för att öppna google callback i browser
}
