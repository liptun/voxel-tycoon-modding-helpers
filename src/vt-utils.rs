mod commands;
mod utils;

use clap::{Parser, Subcommand};
use commands::export::ExportArgs;

#[derive(Parser)]
#[command(version, about = "CLI tools for Voxel Tycoon mod makers", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Export(ExportArgs),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Export(args) => commands::export::run(args).expect("Error"),
    }
}
