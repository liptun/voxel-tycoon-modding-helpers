use crate::utils::get_colors_from_meta::MaterialProperty;

pub fn get_filename_for_material_property(property: &MaterialProperty) -> String {
    match property {
        MaterialProperty::Color => String::from("palette_color.png"),
        MaterialProperty::CompanyTint => String::from("palette_company_tint.png"),
        MaterialProperty::Specular => String::from("palette_specular.png"),
        MaterialProperty::Smoothness => String::from("palette_smoothness.png"),
        MaterialProperty::Glassiness => String::from("palette_glassiness.png"),
        MaterialProperty::Emission => String::from("palette_emission.png"),
    }
}
