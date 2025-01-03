mod json_parse;
mod save_image;
mod hex_to_rgb;

use clap::Parser;
use json_parse::parse_material_json;
use save_image::save_image;
use std::{fs, path::PathBuf};

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

    println!("--------------------------------------\n{:?}\n--------------------------------------", args);

    match fs::read_to_string(&args.file) {
        Ok(content) => {
            if let Ok(mat) = parse_material_json(&content) {
                save_image(&mat, &args.output.into());
            }
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
}
