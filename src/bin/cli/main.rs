pub mod parser;

use kons_coin::database;
use kons_coin::database::crud;

use clap::{ArgGroup, Parser, Subcommand};

/// CLI to interact with database
#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about, 
    long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage a user
    #[command(
        group(
            ArgGroup::new("action")
                .required(true) // must specify exactly one action
                .multiple(false)),
    )]
    User {
        /// Phone number identifying the user
        phone: String,
    
        /// Remove the user
        #[arg(short = 'r', long = "remove", group = "action")]
        remove: bool,
    
        /// Create a user with this name
        #[arg(short = 'c', long = "create", value_name = "NAME", group = "action")]
        create: Option<String>,
    
        /// Show or update the balance
        #[arg(short = 'b', long = "balance", value_name = "BALANCE", group = "action")]
        balance: Option<Option<f32>>,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let pool = database::init_database().await
        .expect("Could not connect to database");

    match cli.command {
        Commands::User { phone, remove, create, balance } => {
            if let Some(name) = create {
                let _ = crud::create_user(&pool, &name, &phone).await
                    .expect("Could not create user");
                return;
            }

            let user = crud::get_user(&pool, None, Some(phone)).await
                .expect("Could not find user");
            
            if remove {
                let _ = crud::delete_user(&pool, user.id).await;
                return;
            }
            match balance {
                Some(Some(b)) => { let _ = crud::update_balance(&pool, user.id, b).await; },
                Some(None) => { println!("{}", user.balance); },
                _ => {}
            };
        }
    }
}
