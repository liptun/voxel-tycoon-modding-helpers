use super::json_parse::{MaterialSchema, VTMetaSchema};

pub type VTPalette = Vec<MaterialSchema>;

pub fn get_variants_from_meta(meta: &VTMetaSchema) -> Option<Vec<String>> {
    let mut variant_names: Vec<String> = Vec::new();
    if let Some(variants) = &meta.variants {
        for (key, _) in variants {
            variant_names.push(key.to_owned());
        }
        if variant_names.len() > 0 {
            return Some(variant_names);
        }
    }

    None
}

pub fn get_palette_from_meta(meta: &VTMetaSchema, variant: Option<String>) -> VTPalette {
    let mut palette: VTPalette = meta.materials.clone();

    println!("Variants {:?}", get_variants_from_meta(meta));

    palette
}
