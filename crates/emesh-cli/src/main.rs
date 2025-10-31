use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod error;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test { output_path } => {
            commands::test::run(output_path)?;
        }
    }

    Ok(())
}
