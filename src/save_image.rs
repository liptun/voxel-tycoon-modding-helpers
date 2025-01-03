use std::path::PathBuf;

use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

use crate::hex_to_rgb::hex_to_rgb;
use crate::json_parse::VTMetaSchema;

pub enum SaveImageError {
    SaveError,
}

pub fn save_image(meta: &VTMetaSchema, output_path: &PathBuf) -> Result<(), SaveImageError> {
    println!("{:?} {:?}", output_path, meta.materials.len());

    let tile_size = 16;

    let size = if meta.materials.len() == 16 { 64 } else { 128 };

    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(size, size);

    let mut start_x = 0;
    let mut start_y = 0;

    for (index, mat) in meta.materials.iter().enumerate() {
        println!("material index {} diffuse is {}", index, mat.color);
        println!("x {}, y {}", start_x, start_y);

        let color = hex_to_rgb(&mat.color).unwrap_or([0, 0, 0]);
        draw_filled_rect_mut(
            &mut img,
            Rect::at(start_x, start_y).of_size(tile_size, tile_size),
            Rgb(color),
        );
        start_x += tile_size as i32;
        if start_x >= size as i32 {
            start_x = 0;
            start_y += tile_size as i32;
        }
    }

    let mut test_file_path = output_path.clone();
    test_file_path.push("test_file.png");

    match img.save(test_file_path) {
        Ok(()) => Ok(()),
        Err(_) => Err(SaveImageError::SaveError),
    }
}
