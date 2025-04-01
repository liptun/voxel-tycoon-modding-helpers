use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialSchema {
    pub color: String,
    pub company_tint: u8,
    pub emission: u8,
    pub glassiness: u8,
    pub smoothness: u8,
    pub specular: u8,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct VariantSchema {
    pub materials: HashMap<u8, MaterialSchema>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct VTMetaSchema {
    pub materials: Vec<MaterialSchema>,
    pub variants: Option<HashMap<String, VariantSchema>>
}

#[derive(Debug)]
pub enum VTMetaReadError {
    ParseError,
}

pub fn parse_material_json(file_content: &str) -> Result<VTMetaSchema, VTMetaReadError> {
    match serde_json::from_str::<VTMetaSchema>(file_content) {
        Ok(meta) => Ok(meta),
        Err(_) => Err(VTMetaReadError::ParseError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_material_json_error() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        assert!(matches!(
            parse_material_json(data),
            Err(VTMetaReadError::ParseError)
        ));
    }

    #[test]
    fn test_parse_material_json_error_missing_field_in_single_material() {
        let data = r#"
        {
            "Materials": [
                {
                  "Color": "424242",
                  "CompanyTint": 21,
                  "Emission": 37,
                  "Glassiness": 69,
                  "Smoothness": 66
                }
            ]
        }"#;

        assert!(matches!(
            parse_material_json(data),
            Err(VTMetaReadError::ParseError)
        ));
    }

    #[test]
    fn test_parse_material_json_error_invalid_field_in_single_material() {
        let data = r#"
        {
            "Materials": [
                {
                  "Color": "424242",
                  "CompanyTint": 21,
                  "Emission": 37,
                  "Glassiness": 69,
                  "Smoothness": 66,
                  "Specular": "13"
                }
            ]
        }"#;

        assert!(matches!(
            parse_material_json(data),
            Err(VTMetaReadError::ParseError)
        ));
    }

    #[test]
    fn test_parse_material_json_success() {
        let data = r#"
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
            ]
        }"#;

        let material_data = parse_material_json(data).expect("Should parse valid JSON");

        assert_eq!(material_data.materials.len(), 1);

        let _expected_material = MaterialSchema {
            color: "424242".to_string(),
            company_tint: 21,
            emission: 37,
            glassiness: 69,
            smoothness: 66,
            specular: 13,
        };
        assert!(matches!(
            material_data
                .materials
                .get(0)
                .expect("Should get first material"),
            _expected_material
        ));
    }
}
