use crate::utils::get_colors_from_meta::{get_colors_from_palette, MaterialProperty};
use crate::utils::json_parse::{parse_material_json, VTMetaReadError};
use crate::utils::palette::get_palette_from_meta;
use crate::utils::save_image::{save_image, SaveImageSuccess};
use clap::Parser;
use std::path::PathBuf;
use std::{collections::HashSet, fs};

#[derive(Hash, PartialEq, Eq, Debug)]
enum ExportOperation {
    Export(MaterialProperty),
}

use ExportOperation::Export;
use MaterialProperty::*;

type ExportOperations = HashSet<ExportOperation>;

#[derive(Parser)]
#[command(
    about = "Export texture images from .meta files.",
    long_about = "Creates material images from .meta files for usage in 3D editor software. Usefull for preview of exporting colored 3D models"
)]
pub struct ExportArgs {
    input_file: PathBuf,

    #[arg(default_value_t = String::from("."))]
    output_directory: String,

    #[arg(short, long)]
    filename: Option<String>,

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

    #[arg(long)]
    variant: Option<String>,

    #[arg(short, long, default_value_t = false)]
    all: bool,
}

fn get_filename_from_path(path: &PathBuf) -> String {
    if let Some(stem) = path.file_stem() {
        stem.to_string_lossy().to_string()
    } else {
        "palette".to_string()
    }
}

#[derive(Debug)]
pub enum ExportError {
    FileRead,
    JsonParse,
    NoOperations,
    InvalidVariantName(String),
}

impl From<std::io::Error> for ExportError {
    fn from(_e: std::io::Error) -> Self {
        ExportError::FileRead
    }
}

impl From<VTMetaReadError> for ExportError {
    fn from(_e: VTMetaReadError) -> Self {
        ExportError::JsonParse
    }
}

struct ProcessArgs {
    color: bool,
    company_tint: bool,
    emission: bool,
    glassiness: bool,
    smoothness: bool,
    specular: bool,
    all: bool,
}

fn process_operations(args: &ProcessArgs) -> Result<ExportOperations, ExportError> {
    let mut operations: ExportOperations = HashSet::new();
    if args.color {
        operations.insert(Export(Color));
    }
    if args.company_tint {
        operations.insert(Export(CompanyTint));
    }
    if args.emission {
        operations.insert(Export(Emission));
    }
    if args.glassiness {
        operations.insert(Export(Glassiness));
    }
    if args.smoothness {
        operations.insert(Export(Smoothness));
    }
    if args.specular {
        operations.insert(Export(Specular));
    }
    if args.all {
        operations.insert(Export(Color));
        operations.insert(Export(CompanyTint));
        operations.insert(Export(Emission));
        operations.insert(Export(Glassiness));
        operations.insert(Export(Smoothness));
        operations.insert(Export(Specular));
    }

    if operations.len() == 0 {
        return Err(ExportError::NoOperations);
    }
    Ok(operations)
}

pub fn run(args: ExportArgs) -> Result<(), ExportError> {
    let filename = if let Some(input_filename) = args.filename {
        input_filename
    } else {
        get_filename_from_path(&args.input_file)
    };

    let content = fs::read_to_string(&args.input_file)?;

    let meta = parse_material_json(&content)?;

    let palette = if let Ok(palette) = get_palette_from_meta(&meta, &args.variant) {
        palette
    } else {
        return Err(ExportError::InvalidVariantName(args.variant.unwrap_or("".to_owned())));
    };

    let process_args = ProcessArgs {
        color: args.color,
        company_tint: args.company_tint,
        emission: args.emission,
        glassiness: args.glassiness,
        smoothness: args.smoothness,
        specular: args.specular,
        all: args.all,
    };
    let operations = process_operations(&process_args)?;

    for operation in operations {
        let ExportOperation::Export(material_type) = operation;
        let colors = get_colors_from_palette(&palette, &material_type);
        let full_filename = format!("{}-{}.png", &filename, &material_type);
        let mut output_directory: PathBuf = args.output_directory.clone().into();
        match save_image(&colors, &mut output_directory, &full_filename) {
            Ok(SaveImageSuccess::SaveOk(message)) => {
                if args.verbose {
                    println!("{}", message);
                }
            }
            Err(e) => println!("Saving failed {:?}", e),
        }
    }
    Ok(())
}
