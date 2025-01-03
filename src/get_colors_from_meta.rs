use crate::{hex_to_rgb::hex_to_rgb, json_parse::VTMetaSchema, save_image::Colors};

pub enum MaterialProperty {
    Color,
    CompanyTint,
    Emission,
    Glassiness,
    Smoothness,
    Specular
}

pub fn get_colors_from_meta(meta: VTMetaSchema, property: MaterialProperty) -> Colors {
    let mut colors: Colors = Vec::new();

    for material in meta.materials.iter() {
        let color = hex_to_rgb(&material.color).unwrap_or([0,0,0]);
        colors.push(color);
    }

    colors
}
