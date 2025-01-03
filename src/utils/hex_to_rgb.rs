use hex::FromHex;

pub enum HexToRgbError {
    ConversionError,
}
pub fn hex_to_rgb(hex: &str) -> Result<[u8; 3], HexToRgbError> {
    let hex = hex.trim_start_matches('#');
    match Vec::from_hex(hex) {
        Ok(bytes) => Ok([bytes[0], bytes[1], bytes[2]]),
        Err(_) => Err(HexToRgbError::ConversionError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_hex_conversion() {
        assert!(matches!(hex_to_rgb("424242"), Ok([66, 66, 66])));
        assert!(matches!(hex_to_rgb("#424242"), Ok([66, 66, 66])));
        assert!(matches!(hex_to_rgb("#ffFFff"), Ok([255, 255, 255])));
    }

    #[test]
    fn test_error_hex_conversion() {
        assert!(matches!(
            hex_to_rgb("42424"),
            Err(HexToRgbError::ConversionError)
        ));
        assert!(matches!(
            hex_to_rgb("42424t"),
            Err(HexToRgbError::ConversionError)
        ));
    }
}
