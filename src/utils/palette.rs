use crate::utils::variants::get_variant_path_from_meta;

use super::{
    json_parse::{MaterialSchema, VTMetaSchema, VariantSchema},
    variants::get_variant_from_meta,
};

pub type VTPalette = Vec<MaterialSchema>;

#[derive(Debug)]
pub enum GetPaletteError {
    VariantNotExist,
    FailedToBuildPalette,
}

fn swap_palette_materials_with_variant(palette: &mut VTPalette, variant: &VariantSchema) {
    for (material_index, material) in variant.materials.iter() {
        let index: usize = *material_index as usize;
        palette[index] = material.clone();
    }
}

pub fn get_palette_from_meta(
    meta: &VTMetaSchema,
    variant: &Option<String>,
) -> Result<VTPalette, GetPaletteError> {
    let mut palette = meta.materials.clone();

    if let Some(variant_name) = variant {
        let variant_path = get_variant_path_from_meta(meta, variant_name)
            .ok_or(GetPaletteError::VariantNotExist)?;
        for variant_path_item in variant_path {
            let variant_colors = get_variant_from_meta(meta, &variant_path_item)
                .ok_or(GetPaletteError::FailedToBuildPalette)?;
            swap_palette_materials_with_variant(&mut palette, variant_colors);
        }
    }

    Ok(palette)
}

#[cfg(test)]
mod tests {
    use crate::utils::json_parse::parse_material_json;

    use super::*;

    fn get_test_data_with_variants() -> String {
        r#"
        {
          "Materials": [
            {
              "Color": "424242",
              "CompanyTint": 21,
              "Emission": 37,
              "Glassiness": 69,
              "Smoothness": 66,
              "Specular": 13
            },
            {
              "Color": "ffa500",
              "CompanyTint": 0,
              "Emission": 0,
              "Glassiness": 0,
              "Smoothness": 0,
              "Specular": 0
            }
          ],
          "Variants": {
            "Foo": {
              "Materials": {
                "1": {
                  "Color": "ff0000",
                  "CompanyTint": 21,
                  "Emission": 37,
                  "Glassiness": 69,
                  "Smoothness": 66,
                  "Specular": 13
                }
              }
            },
            "Bar": {
              "Materials": {
                "1": {
                  "Color": "00ff00",
                  "CompanyTint": 21,
                  "Emission": 37,
                  "Glassiness": 69,
                  "Smoothness": 66,
                  "Specular": 13
                }
              }
            }
          }
        }"#
        .to_string()
    }

    #[test]
    fn test_get_variants_from_meta() {
        let meta = parse_material_json(&get_test_data_with_variants()).expect("Should create meta");

        let search_variant = "Foo".to_string();
        let variant = get_variant_from_meta(&meta, &search_variant).expect("Should get variant");

        let index: u8 = 1;
        assert_eq!(
            variant
                .materials
                .get(&index)
                .expect("Should get material")
                .color,
            "ff0000".to_string()
        )
    }

    #[test]
    fn test_get_palette_from_meta() {
        let meta = parse_material_json(&get_test_data_with_variants()).expect("Should create meta");
        let search_variant = "Foo".to_string();

        let palette =
            get_palette_from_meta(&meta, &Some(search_variant)).expect("Should get palette");

        assert_eq!(palette[0].color, "424242");

        assert_eq!(palette[1].color, "ff0000");
        assert_ne!(palette[1].color, "ffa500");
    }
}
