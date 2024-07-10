use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a mesh to a graphics representation
    Test {
        /// Path to the output directory
        #[clap(short, long)]
        output_path: String,
    },
}
