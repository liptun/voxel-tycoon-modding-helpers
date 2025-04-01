use core::fmt;

use crate::utils::{hex_to_rgb::hex_to_rgb, save_image::Colors};

use super::palette::VTPalette;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum MaterialProperty {
    Color,
    CompanyTint,
    Emission,
    Glassiness,
    Smoothness,
    Specular,
}

impl fmt::Display for MaterialProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display: &str = match self {
            MaterialProperty::Color => "color",
            MaterialProperty::CompanyTint => "company-tint",
            MaterialProperty::Emission => "emission",
            MaterialProperty::Glassiness => "glassiness",
            MaterialProperty::Smoothness => "smoothness",
            MaterialProperty::Specular => "specular",
        };
        write!(f, "{}", display)
    }
}

fn value_to_rgb(value: u8) -> [u8; 3] {
    [value, value, value]
}

pub fn get_colors_from_palette(palette: &VTPalette, property: &MaterialProperty) -> Colors {
    let mut colors: Colors = Vec::new();

    for material in palette.iter() {
        let color: [u8; 3] = match property {
            MaterialProperty::Color => hex_to_rgb(&material.color).unwrap_or([0, 0, 0]),
            MaterialProperty::CompanyTint => value_to_rgb(material.company_tint),
            MaterialProperty::Emission => value_to_rgb(material.emission),
            MaterialProperty::Glassiness => value_to_rgb(material.glassiness),
            MaterialProperty::Smoothness => value_to_rgb(material.smoothness),
            MaterialProperty::Specular => value_to_rgb(material.specular),
        };
        colors.push(color);
    }

    colors
}
