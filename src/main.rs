mod json_parse;
use std::{fs, path::PathBuf};

use clap::Parser;
use json_parse::parse_material_json;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[arg(default_value_t = String::from("."))]
    output: String,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    match fs::read_to_string(&args.file) {
        Ok(content) => {
            if let Ok(mat) = parse_material_json(&content) {
                println!("{:?}", mat);
                println!("Color palette size: {}", mat.materials.len());
            }
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
}
