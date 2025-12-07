/// Parse an enum value - handles both hex strings (with 0x prefix) and decimal strings
/// Returns i64 to support negative values
pub fn parse_enum_value(value_str: &str) -> Result<i64, String> {
    let value_str = value_str.trim();

    // Try to parse as hex if it has 0x prefix
    if let Some(hex_part) = value_str
        .strip_prefix("0x")
        .or_else(|| value_str.strip_prefix("0X"))
    {
        i64::from_str_radix(hex_part, 16)
            .map_err(|e| format!("Failed to parse hex '{}': {}", value_str, e))
    } else {
        // Otherwise try to parse as decimal
        value_str
            .parse::<i64>()
            .map_err(|e| format!("Failed to parse number '{}': {}", value_str, e))
    }
}

/// Format an i64 value as a hex string with leading zero for single-digit values
/// Examples: 4 -> "0x04", 171 -> "0xAB", -100 -> "-100"
pub fn format_hex_value(value: i64) -> String {
    if value < 0 {
        // Negative values use decimal format
        format!("{}", value)
    } else if value < 256 {
        // Small positive values (< 256) use 2-digit hex with leading zero
        format!("0x{:02X}", value)
    } else {
        // Larger values use natural hex width (no padding)
        format!("0x{:X}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_enum_value_basic() {
        assert_eq!(parse_enum_value("0x04"), Ok(4));
        assert_eq!(parse_enum_value("0x4"), Ok(4));
        assert_eq!(parse_enum_value("0xAB"), Ok(171));
        assert_eq!(parse_enum_value("0xab"), Ok(171));
    }

    #[test]
    fn test_parse_enum_value_uppercase_prefix() {
        assert_eq!(parse_enum_value("0X04"), Ok(4));
        assert_eq!(parse_enum_value("0XFF"), Ok(255));
    }

    #[test]
    fn test_parse_enum_value_with_whitespace() {
        assert_eq!(parse_enum_value("  0x10  "), Ok(16));
        assert_eq!(parse_enum_value("\t0xA\n"), Ok(10));
    }

    #[test]
    fn test_parse_enum_value_large_values() {
        assert_eq!(parse_enum_value("0x40000002"), Ok(0x40000002));
        assert_eq!(parse_enum_value("0x08000000"), Ok(0x08000000));
    }

    #[test]
    fn test_parse_enum_value_zero() {
        assert_eq!(parse_enum_value("0x0"), Ok(0));
        assert_eq!(parse_enum_value("0x00"), Ok(0));
        assert_eq!(parse_enum_value("0x0000"), Ok(0));
    }

    #[test]
    fn test_parse_enum_value_decimal() {
        assert_eq!(parse_enum_value("5"), Ok(5));
        assert_eq!(parse_enum_value("123"), Ok(123));
        assert_eq!(parse_enum_value("-100"), Ok(-100));
        assert_eq!(parse_enum_value("0"), Ok(0));
    }

    #[test]
    fn test_parse_enum_value_invalid() {
        assert!(parse_enum_value("hello").is_err());
        assert!(parse_enum_value("12.5").is_err());
        assert!(parse_enum_value("").is_err());
    }

    #[test]
    fn test_parse_enum_value_invalid_hex() {
        assert!(parse_enum_value("0xZZ").is_err());
        assert!(parse_enum_value("0x").is_err());
        assert!(parse_enum_value("0xG").is_err());
    }

    #[test]
    fn test_parse_enum_value_negative_hex() {
        // Negative hex values with 0x prefix are not supported
        assert!(parse_enum_value("-0x10").is_err());
    }
}
