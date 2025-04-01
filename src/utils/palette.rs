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
