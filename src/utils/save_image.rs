use std::path::PathBuf;

use image::{imageops, ImageBuffer, Rgb};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

#[derive(Debug)]
pub enum SaveImageSuccess {
    SaveOk(String),
}

#[derive(Debug)]
pub enum SaveImageError {
    SaveError,
    InvalidInputLength,
}

pub type Colors = Vec<[u8; 3]>;

pub fn save_image(
    colors: &Colors,
    output_path: &mut PathBuf,
    filename: &str,
) -> Result<SaveImageSuccess, SaveImageError> {
    if colors.len() != 16 && colors.len() != 64 {
        return Err(SaveImageError::InvalidInputLength);
    }
    let tile_size = 16;

    let size = if colors.len() <= 16 {
        tile_size * 4
    } else {
        tile_size * 8
    };

    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(size, size);

    let mut start_x = 0;
    let mut start_y = 0;

    for color in colors.iter() {
        draw_filled_rect_mut(
            &mut img,
            Rect::at(start_x, start_y).of_size(tile_size, tile_size),
            Rgb(*color),
        );
        start_x += tile_size as i32;
        if start_x >= size as i32 {
            start_x = 0;
            start_y += tile_size as i32;
        }
    }

    let corrected_orientation_img = imageops::flip_vertical(&img);

    output_path.push(filename);

    let str_output_path: &str = &output_path.to_string_lossy();

    match corrected_orientation_img.save(&output_path) {
        Ok(()) => Ok(SaveImageSuccess::SaveOk(format!(
            "Succesfully saved {}",
            str_output_path
        ))),
        Err(_) => Err(SaveImageError::SaveError),
    }
}
