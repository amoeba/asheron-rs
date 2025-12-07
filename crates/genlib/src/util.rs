/// Parse a hex string to i64
pub fn parse_hex_string(hex_str: &str) -> Result<i64, String> {
    let hex_str = hex_str.trim();

    let hex_part = hex_str
        .strip_prefix("0x")
        .or_else(|| hex_str.strip_prefix("0X"))
        .ok_or_else(|| format!("Not a hex string: '{}'", hex_str))?;

    i64::from_str_radix(hex_part, 16)
        .map_err(|e| format!("Failed to parse hex '{}': {}", hex_str, e))
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
    fn test_parse_hex_string_basic() {
        assert_eq!(parse_hex_string("0x04"), Ok(4));
        assert_eq!(parse_hex_string("0x4"), Ok(4));
        assert_eq!(parse_hex_string("0xAB"), Ok(171));
        assert_eq!(parse_hex_string("0xab"), Ok(171));
    }

    #[test]
    fn test_parse_hex_string_uppercase_prefix() {
        assert_eq!(parse_hex_string("0X04"), Ok(4));
        assert_eq!(parse_hex_string("0XFF"), Ok(255));
    }

    #[test]
    fn test_parse_hex_string_with_whitespace() {
        assert_eq!(parse_hex_string("  0x10  "), Ok(16));
        assert_eq!(parse_hex_string("\t0xA\n"), Ok(10));
    }

    #[test]
    fn test_parse_hex_string_large_values() {
        assert_eq!(parse_hex_string("0x40000002"), Ok(0x40000002));
        assert_eq!(parse_hex_string("0x08000000"), Ok(0x08000000));
    }

    #[test]
    fn test_parse_hex_string_zero() {
        assert_eq!(parse_hex_string("0x0"), Ok(0));
        assert_eq!(parse_hex_string("0x00"), Ok(0));
        assert_eq!(parse_hex_string("0x0000"), Ok(0));
    }

    #[test]
    fn test_parse_hex_string_non_hex() {
        assert!(parse_hex_string("123").is_err());
        assert!(parse_hex_string("-100").is_err());
        assert!(parse_hex_string("hello").is_err());
    }

    #[test]
    fn test_parse_hex_string_invalid_hex() {
        assert!(parse_hex_string("0xZZ").is_err());
        assert!(parse_hex_string("0x").is_err());
        assert!(parse_hex_string("0xG").is_err());
    }

    #[test]
    fn test_parse_hex_string_negative() {
        // Negative hex values are not supported (no 0x prefix for negatives)
        assert!(parse_hex_string("-0x10").is_err());
    }
}
