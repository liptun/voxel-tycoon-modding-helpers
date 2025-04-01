use super::json_parse::{MaterialSchema, VTMetaSchema, VariantSchema};

pub type VTPalette = Vec<MaterialSchema>;

pub fn get_variants_names_from_meta(meta: &VTMetaSchema) -> Option<Vec<String>> {
    let mut variant_names: Vec<String> = Vec::new();
    if let Some(variants) = &meta.variants {
        for (key, _) in variants {
            variant_names.push(key.to_owned());
        }
        if !variant_names.is_empty() {
            return Some(variant_names);
        }
    }

    None
}

pub fn get_variant_from_meta<'a>(
    meta: &'a VTMetaSchema,
    variant: &'a String,
) -> Option<&'a VariantSchema> {
    if let Some(meta_variants) = &meta.variants {
        if let Some(variant_colors) = &meta_variants.get(variant) {
            return Some(variant_colors);
        }
    }

    None
}

#[derive(Debug)]
pub enum GetPaletteError {
    VariantNotExist,
}

pub fn get_palette_from_meta(
    meta: &VTMetaSchema,
    variant: &Option<String>,
) -> Result<VTPalette, GetPaletteError> {
    let mut palette = meta.materials.clone();

    if let Some(variant_name) = variant {
        if let Some(variant_colors) = get_variant_from_meta(meta, variant_name) {
            for (material_index, material) in variant_colors.materials.iter() {
                let index: usize = *material_index as usize;
                palette[index] = material.clone();
            }
        } else {
            return Err(GetPaletteError::VariantNotExist);
        }
    }

    Ok(palette)
}

#[cfg(test)]
mod tests {
    use crate::utils::json_parse::parse_material_json;

    use super::*;

    fn get_test_data_without_variants() -> String {
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
            }
          ],
          "Variants": null
        }"#
        .to_string()
    }

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
    fn test_get_variants_names() {
        let meta = parse_material_json(&get_test_data_with_variants()).expect("Should create meta");
        let mut variants = get_variants_names_from_meta(&meta).expect("Should get variants");
        variants.sort();

        let mut expected_variants = vec!["Foo".to_string(), "Bar".to_string()];
        expected_variants.sort();
        assert_eq!(variants, expected_variants);

        let meta_without =
            parse_material_json(&get_test_data_without_variants()).expect("Should create meta");
        assert_eq!(get_variants_names_from_meta(&meta_without), None);
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
