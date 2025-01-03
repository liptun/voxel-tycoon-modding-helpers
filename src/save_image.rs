use std::path::PathBuf;

use image::{ImageBuffer, Rgba};

use crate::json_parse::VTMetaSchema;

pub fn save_image(meta: &VTMetaSchema, output_path: &PathBuf) {
    println!("{:?} {:?}", output_path, meta.materials.len());

    let tile_size: u32 = 16;

    let size: u32 = if meta.materials.len() == 16 { 64 } else { 128 };

    let mut img = ImageBuffer::new(size, size);

    let red = Rgba::<u8>([255, 0, 0, 255]);

    for pixel in img.pixels_mut() {
        *pixel = red;
    }

    let mut test_file_path = output_path.clone();
    test_file_path.push("test_file.png");

    img.save(test_file_path).expect("Failed saving file")
}
