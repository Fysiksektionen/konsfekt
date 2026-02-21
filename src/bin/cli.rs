use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Auth, // Login, Logout etc
}
#[tokio::main]
async fn main() {
    // Todo använd reqwest för att logga in på backend och spara cookies
    // använd open för att öppna google callback i browser
}
