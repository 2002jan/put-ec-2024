use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// Problem file path
    pub file: String,

    /// What kind of test to run
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand)]
pub enum Command {
    Task1
}

impl Args {
    pub fn build() -> Args {
        Args::parse()
    }
}