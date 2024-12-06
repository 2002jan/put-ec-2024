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
    Task1,
    Task2,
    Task3,
    Task4,
    Task5,
    Task6,
    Task7
}

impl Args {
    pub fn build() -> Args {
        Args::parse()
    }
}