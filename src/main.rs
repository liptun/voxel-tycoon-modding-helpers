mod hex_to_rgb;
mod json_parse;
mod save_image;
mod get_colors_from_meta;

use clap::Parser;
use get_colors_from_meta::{get_colors_from_meta, MaterialProperty};
use json_parse::parse_material_json;
use save_image::{save_image, Colors};
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

    println!(
        "--------------------------------------\n{:?}\n--------------------------------------",
        args
    );

    match fs::read_to_string(&args.file) {
        Ok(content) => {
            if let Ok(meta) = parse_material_json(&content) {
                let colors = get_colors_from_meta(meta, MaterialProperty::Color);

                match save_image(&colors, &args.output.into(), "test.png") {
                    Ok(ok_message) => println!("{:?}", ok_message),
                    Err(e) => println!("{:?}", e)
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
}
