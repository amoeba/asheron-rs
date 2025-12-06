/// Convert PascalCase/camelCase to snake_case
pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = name.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            // Add underscore before uppercase if:
            // - Not the first character AND
            // - Previous character was lowercase, OR
            // - Next character is lowercase (handles "YGrid" -> "y_grid")
            if i > 0 {
                let prev_is_lower = chars[i - 1].is_lowercase();
                let next_is_lower = chars.get(i + 1).is_some_and(|c| c.is_lowercase());
                if prev_is_lower || next_is_lower {
                    result.push('_');
                }
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }

    result
}

const RUST_RESERVED_WORDS: &[&str] = &["self", "Self", "type"];

/// Check if a field name is a Rust reserved word
pub fn is_reserved_word(name: &str) -> bool {
    RUST_RESERVED_WORDS.contains(&name)
}

/// Convert a field name to a safe Rust identifier in snake_case
pub fn safe_field_name(name: &str) -> (String, bool) {
    let snake_case = to_snake_case(name);

    if is_reserved_word(&snake_case) {
        let safe_name = format!("{snake_case}_");
        (safe_name, true)
    } else {
        let needs_rename = name != snake_case;
        (snake_case, needs_rename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_simple() {
        assert_eq!(to_snake_case("PlayerId"), "player_id");
        assert_eq!(to_snake_case("Team"), "team");
        assert_eq!(to_snake_case("IdPieceToMove"), "id_piece_to_move");
    }

    #[test]
    fn test_to_snake_case_consecutive_capitals() {
        assert_eq!(to_snake_case("YGrid"), "y_grid");
        assert_eq!(to_snake_case("XTo"), "x_to");
        assert_eq!(to_snake_case("YTo"), "y_to");
    }

    #[test]
    fn test_to_snake_case_all_caps() {
        assert_eq!(to_snake_case("ID"), "id");
        assert_eq!(to_snake_case("URL"), "url");
    }

    #[test]
    fn test_to_snake_case_already_snake() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
        assert_eq!(to_snake_case("test"), "test");
    }

    #[test]
    fn test_to_snake_case_mixed() {
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
        assert_eq!(to_snake_case("getHTTPResponse"), "get_http_response");
    }

    
}
