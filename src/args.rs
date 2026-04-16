use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub mode: Mode,
    #[arg(long = "static")]
    pub static_frontend: bool
}

#[derive(Clone, ValueEnum, PartialEq)]
pub enum Mode {
    Local,
    Prod,
    Tunnel,
}
