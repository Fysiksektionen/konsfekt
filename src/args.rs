use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long = "local")]
    pub run_locally: bool,
    #[arg(long = "static")]
    pub static_frontend: bool
}
