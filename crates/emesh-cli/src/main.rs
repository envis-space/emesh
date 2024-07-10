use crate::arguments::{Arguments, Commands};
use clap::Parser;
use std::path::PathBuf;

mod arguments;
mod commands;

fn main() {
    tracing_subscriber::fmt::init();
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::Test { output_path } => {
            let output_directory_path = PathBuf::from(&output_path);

            commands::test::run(output_directory_path);
        }
    }
}
