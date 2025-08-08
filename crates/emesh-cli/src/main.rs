use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod cli;
mod commands;
mod error;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test { output_path } => {
            let output_directory_path = PathBuf::from(&output_path);

            commands::test::run(output_directory_path)?;
        }
    }

    Ok(())
}
