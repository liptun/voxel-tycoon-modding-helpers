use crate::utils::get_colors_from_meta::{get_colors_from_meta, MaterialProperty};
use crate::utils::get_filename::get_filename_for_material_property;
use crate::utils::json_parse::parse_material_json;
use crate::utils::save_image::{save_image, SaveImageSuccess};
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use std::{collections::HashSet, fs, process};

#[derive(Hash, PartialEq, Eq, Debug)]
enum QueueOperation {
    Export(MaterialProperty),
}

type QueueExport = HashSet<QueueOperation>;

#[derive(Parser)]
#[command(
    about = "Export texture images from .meta files.",
    long_about = "Creates material images from .meta files for usage in 3D editor software. Usefull for preview of exporting colored 3D models"
)]
pub struct ExportArgs {
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

pub fn run(args: ExportArgs) -> Result<(), Box<dyn Error>> {
    match fs::read_to_string(&args.file) {
        Ok(content) => {
            if let Ok(meta) = parse_material_json(&content) {
                let mut queue: QueueExport = HashSet::new();
                if args.color {
                    queue.insert(QueueOperation::Export(MaterialProperty::Color));
                }
                if args.company_tint {
                    queue.insert(QueueOperation::Export(MaterialProperty::CompanyTint));
                }
                if args.emission {
                    queue.insert(QueueOperation::Export(MaterialProperty::Emission));
                }
                if args.glassiness {
                    queue.insert(QueueOperation::Export(MaterialProperty::Glassiness));
                }
                if args.smoothness {
                    queue.insert(QueueOperation::Export(MaterialProperty::Smoothness));
                }
                if args.specular {
                    queue.insert(QueueOperation::Export(MaterialProperty::Specular));
                }
                if args.all {
                    queue.insert(QueueOperation::Export(MaterialProperty::Color));
                    queue.insert(QueueOperation::Export(MaterialProperty::CompanyTint));
                    queue.insert(QueueOperation::Export(MaterialProperty::Emission));
                    queue.insert(QueueOperation::Export(MaterialProperty::Glassiness));
                    queue.insert(QueueOperation::Export(MaterialProperty::Smoothness));
                    queue.insert(QueueOperation::Export(MaterialProperty::Specular));
                }

                if queue.len() == 0 {
                    println!("Specify export operation. Use -h for help");
                    process::exit(2);
                }

                for operation in queue {
                    let QueueOperation::Export(material_type) = operation;
                    let colors = get_colors_from_meta(&meta, &material_type);
                    match save_image(
                        &colors,
                        &args.output.clone().into(),
                        &get_filename_for_material_property(&material_type),
                    ) {
                        Ok(SaveImageSuccess::SaveOk(message)) => {
                            if args.verbose {
                                println!("{}", message);
                            }
                        }
                        Err(e) => println!("Saving failed {:?}", e),
                    }
                }
            } else {
                println!("Invalid input file. Verify if you provided .meta file");
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
        }
    }
    Ok(())
}
