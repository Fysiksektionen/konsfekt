pub mod parser;

use clap::{Parser, Subcommand, Args};
use kons_coin::database;
use kons_coin::database::crud;

/// CLI to interact with database
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    AddUser(AddUserCommand)
}

#[derive(Debug, Args)]
struct AddUserCommand {
    name: String,
    phone: String
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let pool = database::init_database().await.expect("Could not connect to database");
    match cli.action {
        Action::AddUser(input) => {
            let _ = crud::create_user(&pool, &input.name, &input.phone).await;
        }
    }
}
