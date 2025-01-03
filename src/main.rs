mod commands;

mod get_colors_from_meta;
mod get_filename;
mod hex_to_rgb;
mod json_parse;
mod save_image;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    file: PathBuf,

    #[arg(default_value_t = String::from("."))]
    output: String,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long, default_value_t = false)]
    color: bool,

    #[arg(short = 't', long, default_value_t = false)]
    company_tint: bool,

    #[arg(short, long, default_value_t = false)]
    emission: bool,

    #[arg(short, long, default_value_t = false)]
    glassiness: bool,

    #[arg(short, long, default_value_t = false)]
    smoothness: bool,

    #[arg(short = 'r', long, default_value_t = false)]
    specular: bool,

    #[arg(short, long, default_value_t = false)]
    all: bool,
}

fn main() {
    let args = CliArgs::parse();

    commands::export::run(args).expect("error");
}
