mod commands;
mod utils;

use clap::{Parser, Subcommand};
use commands::export::{ExportArgs, ExportError};

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
        Commands::Export(args) => run_export(args),
    }
}

fn run_export(args: ExportArgs) {
    match commands::export::run(args) {
        Err(ExportError::FileRead) => println!("Failed to read the file"),
        Err(ExportError::NoOperations) => println!(
            "Specify export operation. Use -h for help or if you want export all textures use -a"
        ),
        Err(ExportError::JsonParse) => {
            println!("Invalid input file. Verify if you provided .meta file.",)
        }
        Err(ExportError::InvalidVariantName((variant, available_variants))) => {
            println!("Invalid variant name: {}", variant);
            if let Some(variants_list) = available_variants {
                println!("Available variants: {}", variants_list.join(", "));
            }
        }
        Ok(_) => {}
    }
}
