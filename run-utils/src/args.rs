use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// What kind of test to run
    #[arg(value_enum)]
    pub command: Command,

    /// Problem file path
    pub file: String,

    pub outputs_folder: Option<String>
}

#[derive(Clone, ValueEnum)]
pub enum Command {
    Task1
}

impl Args {
    pub fn build() -> Args {
        Args::parse()
    }
}