use anyhow::Result;

/// Parse an opcode filter string to u32
///
/// Supports:
/// - Hex with prefix: "0xF7B1" or "0xf7b1"
/// - Decimal: "63409"
pub fn parse_opcode_filter(s: &str) -> Result<u32> {
    let s = s.trim();

    if s.starts_with("0x") || s.starts_with("0X") {
        // Parse as hex
        u32::from_str_radix(&s[2..], 16)
            .map_err(|e| anyhow::anyhow!("Invalid hex opcode '{s}': {e}"))
    } else {
        // Parse as decimal
        s.parse::<u32>()
            .map_err(|e| anyhow::anyhow!("Invalid decimal opcode '{s}': {e}"))
    }
}

/// Convert opcode string (hex format like "F7B1") to u32
pub fn opcode_str_to_u32(opcode_str: &str) -> Option<u32> {
    u32::from_str_radix(opcode_str, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_hex_with_prefix_uppercase() {
        assert_eq!(parse_opcode_filter("0xF7B1").unwrap(), 0xF7B1);
    }

    #[test]
    fn test_parse_opcode_hex_with_prefix_lowercase() {
        assert_eq!(parse_opcode_filter("0xf7b1").unwrap(), 0xf7b1);
    }

    #[test]
    fn test_parse_opcode_hex_with_prefix_mixed() {
        assert_eq!(parse_opcode_filter("0xF7b1").unwrap(), 0xF7B1);
    }

    #[test]
    fn test_parse_opcode_decimal() {
        assert_eq!(parse_opcode_filter("63409").unwrap(), 0xF7B1);
    }

    #[test]
    fn test_parse_opcode_invalid_hex() {
        assert!(parse_opcode_filter("0xGHIJ").is_err());
    }

    #[test]
    fn test_parse_opcode_invalid_decimal() {
        assert!(parse_opcode_filter("not_a_number").is_err());
    }

    #[test]
    fn test_opcode_str_to_u32() {
        assert_eq!(opcode_str_to_u32("F7B1"), Some(0xF7B1));
        assert_eq!(opcode_str_to_u32("f7b1"), Some(0xf7b1));
        assert_eq!(opcode_str_to_u32("GHIJ"), None);
    }

    // Comprehensive equivalence tests: hex and decimal representations should be identical
    #[test]
    fn test_hex_and_decimal_equivalence_f7b1() {
        // 0xF7B1 = 63409 in decimal
        let hex_result = parse_opcode_filter("0xF7B1").unwrap();
        let hex_lowercase = parse_opcode_filter("0xf7b1").unwrap();
        let decimal_result = parse_opcode_filter("63409").unwrap();

        assert_eq!(hex_result, decimal_result);
        assert_eq!(hex_lowercase, decimal_result);
        assert_eq!(hex_result, 0xF7B1);
    }

    #[test]
    fn test_hex_and_decimal_equivalence_02cd() {
        // 0x02CD = 717 in decimal
        let hex_result = parse_opcode_filter("0x02CD").unwrap();
        let hex_lowercase = parse_opcode_filter("0x02cd").unwrap();
        let decimal_result = parse_opcode_filter("717").unwrap();

        assert_eq!(hex_result, decimal_result);
        assert_eq!(hex_lowercase, decimal_result);
        assert_eq!(hex_result, 0x02CD);
    }

    #[test]
    fn test_hex_and_decimal_equivalence_edge_cases() {
        // Test 0x0000 (0 decimal)
        assert_eq!(
            parse_opcode_filter("0x0000").unwrap(),
            parse_opcode_filter("0").unwrap()
        );

        // Test 0x0001 (1 decimal)
        assert_eq!(
            parse_opcode_filter("0x0001").unwrap(),
            parse_opcode_filter("1").unwrap()
        );

        // Test 0xFFFF (65535 decimal)
        assert_eq!(
            parse_opcode_filter("0xFFFF").unwrap(),
            parse_opcode_filter("65535").unwrap()
        );
    }

    #[test]
    fn test_opcode_str_matches_parsed_filter() {
        // When we parse "0xF7B1" as a filter and convert "F7B1" as opcode string,
        // they should produce the same value
        let parsed = parse_opcode_filter("0xF7B1").unwrap();
        let converted = opcode_str_to_u32("F7B1").unwrap();
        assert_eq!(parsed, converted);

        let parsed_decimal = parse_opcode_filter("63409").unwrap();
        assert_eq!(parsed_decimal, converted);
    }

    #[test]
    fn test_decimal_and_hex_produce_identical_filtering_value() {
        // This test verifies that both input formats would match the same message
        // by ensuring they produce the same u32 value
        let test_opcode_str = "F7B1"; // As it appears in message
        let message_opcode_value = opcode_str_to_u32(test_opcode_str).unwrap();

        // Both ways of filtering should match
        let hex_filter = parse_opcode_filter("0xF7B1").unwrap();
        let decimal_filter = parse_opcode_filter("63409").unwrap();

        assert_eq!(message_opcode_value, hex_filter);
        assert_eq!(message_opcode_value, decimal_filter);
    }
}
