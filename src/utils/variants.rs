use super::json_parse::{VTMetaSchema, VariantSchema, Variants};

fn get_variants_names_from_variant(variant: &VariantSchema) -> Option<Vec<String>> {
    let mut variant_names: Vec<String> = Vec::new();

    if let Some(variants) = &variant.variants {
        for (key, variant) in variants {
            variant_names.push(key.to_owned());
            let nested_names = get_variants_names_from_variant(variant);
            if let Some(nested_keys) = nested_names {
                variant_names.extend(nested_keys);
            }
        }
        return Some(variant_names);
    }

    None
}

pub fn get_variants_names_from_meta(meta: &VTMetaSchema) -> Option<Vec<String>> {
    let mut variant_names: Vec<String> = Vec::new();
    if let Some(variants) = &meta.variants {
        for (key, variant) in variants {
            variant_names.push(key.to_owned());
            let nested_names = get_variants_names_from_variant(variant);
            if let Some(nested_keys) = nested_names {
                variant_names.extend(nested_keys);
            }
        }
        if !variant_names.is_empty() {
            return Some(variant_names);
        }
    }

    None
}

pub type VariantPath = Vec<String>;

fn search_for_variant_path(variants: &Variants, variant_name: &String) -> Option<VariantPath> {
    if let Some(_) = variants.get(variant_name) {
        return Some(vec![variant_name.to_owned()]);
    }

    for (key, sub_variant) in variants {
        if let Some(found) = get_variant_subpath_from_variant(&sub_variant, &variant_name) {
            let mut result = vec![key.to_string()];
            result.extend(found);
            return Some(result);
        }
    }

    None
}

fn get_variant_subpath_from_variant(
    variant: &VariantSchema,
    variant_name: &String,
) -> Option<VariantPath> {
    variant
        .variants
        .as_ref()
        .and_then(|variants| search_for_variant_path(variants, variant_name))
}

pub fn get_variant_path_from_meta(
    meta: &VTMetaSchema,
    variant_name: &String,
) -> Option<VariantPath> {
    meta.variants
        .as_ref()
        .and_then(|variants| search_for_variant_path(variants, variant_name))
}

fn search_for_variant<'a>(
    variants: &'a Variants,
    variant_name: &'a String,
) -> Option<&'a VariantSchema> {
    if let Some(variant) = variants.get(variant_name) {
        return Some(variant);
    }

    for (_, sub_variant) in variants {
        if let Some(found) = get_variant_from_variant(&sub_variant, &variant_name) {
            return Some(found);
        }
    }

    None
}

fn get_variant_from_variant<'a>(
    variant: &'a VariantSchema,
    variant_name: &'a String,
) -> Option<&'a VariantSchema> {
    if let Some(meta_variants) = &variant.variants {
        return search_for_variant(meta_variants, variant_name);
    }

    None
}

pub fn get_variant_from_meta<'a>(
    meta: &'a VTMetaSchema,
    variant_name: &'a String,
) -> Option<&'a VariantSchema> {
    if let Some(meta_variants) = &meta.variants {
        return search_for_variant(meta_variants, variant_name);
    }

    None
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
        r#"{
  "Materials": [
    { "Color": "404040", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "9f1200", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "ffffff", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "404040", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "32a6a6", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "272727", "CompanyTint": 0, "Emission": 0, "Glassiness": 255, "Smoothness": 0, "Specular": 255 },
    { "Color": "c6c6c6", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "f4dda4", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "404040", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "ffffff", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "ffffff", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "f4dda4", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "1ca14a", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "404040", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "c7171e", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
    { "Color": "858585", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 }
  ],
  "Variants": {
    "foo": {
      "Materials": {
        "12": { "Color": "a11c1c", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 }
      },
      "Variants": {
        "bar": {
          "Materials": {
            "15": { "Color": "bb4040", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 }
          },
          "Variants": {
            "lorem": {
              "Materials": {
                "7": { "Color": "505050", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 }
              },
              "Variants": {
                "ipsum": {
                  "Materials": {
                    "11": { "Color": "f0a4f4", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 }
                  },
                  "Variants": {
                    "deep": {
                      "Materials": {
                        "12": { "Color": "a18b1c", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 },
                        "15": { "Color": "ce7837", "CompanyTint": 0, "Emission": 0, "Glassiness": 0, "Smoothness": 0, "Specular": 0 }
                      },
                      "Variants": null
                    }
                  }
                }
              }
            }
          }
        }
      }
    },
    "night": {
      "Materials": {
        "1": { "Color": "9f1200", "CompanyTint": 0, "Emission": 255, "Glassiness": 0, "Smoothness": 191, "Specular": 0 },
        "5": { "Color": "d9ac4c", "CompanyTint": 0, "Emission": 255, "Glassiness": 255, "Smoothness": 160, "Specular": 255 }
      },
      "Variants": null
    }
  }
}"#.to_string()
    }

    #[test]
    fn test_get_variants_names() {
        let meta = parse_material_json(&get_test_data_with_variants()).expect("Should create meta");
        let mut variants = get_variants_names_from_meta(&meta).expect("Should get variants");
        variants.sort();

        let mut expected_variants = vec![
            "foo".to_string(),
            "bar".to_string(),
            "night".to_string(),
            "lorem".to_string(),
            "ipsum".to_string(),
            "deep".to_string(),
        ];
        expected_variants.sort();
        assert_eq!(variants, expected_variants);

        let meta_without =
            parse_material_json(&get_test_data_without_variants()).expect("Should create meta");
        assert_eq!(get_variants_names_from_meta(&meta_without), None);
    }

    #[test]
    fn test_get_variant_path_from_meta_deep_lvl() {
        let meta = parse_material_json(&get_test_data_with_variants()).expect("Should create meta");
        let expected_variants = vec![
            "foo".to_string(),
            "bar".to_string(),
            "lorem".to_string(),
            "ipsum".to_string(),
            "deep".to_string(),
        ];

        let variants =
            get_variant_path_from_meta(&meta, &"deep".to_string()).expect("Should get variants");

        assert_eq!(variants, expected_variants);
    }

    #[test]
    fn test_get_variant_path_from_meta_first_lvl() {
        let meta = parse_material_json(&get_test_data_with_variants()).expect("Should create meta");
        let expected_variants = vec!["night".to_string()];

        let variants =
            get_variant_path_from_meta(&meta, &"night".to_string()).expect("Should get variants");

        assert_eq!(variants, expected_variants);
    }
}
