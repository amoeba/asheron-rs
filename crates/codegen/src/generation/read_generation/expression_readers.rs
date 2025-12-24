use crate::{
    identifiers::{IdentifierType, safe_identifier},
    types::Field,
};

/// Check if a token is a numeric literal (integer, float, or hex)
fn is_numeric_literal(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    // Check for hex literal (0x... or 0X...)
    if token.len() > 2 && (token.starts_with("0x") || token.starts_with("0X")) {
        return token[2..].chars().all(|c| c.is_ascii_hexdigit());
    }

    // Check for decimal integer or float
    // Valid formats: "123", "123.456", ".456"
    let mut has_dot = false;
    for ch in token.chars() {
        if ch == '.' {
            if has_dot {
                return false; // Multiple dots
            }
            has_dot = true;
        } else if !ch.is_numeric() {
            return false;
        }
    }

    true
}

/// Generic expression parser that tokenizes an expression and applies formatting
///
/// # Arguments
/// * `expr` - The expression to parse
/// * `all_fields` - Available fields for token resolution
/// * `is_separator` - Predicate to determine if a character is a separator/operator
/// * `format_token` - Function to format resolved tokens (fields or literals)
/// * `format_separator` - Function to format separator characters
pub fn parse_expression<F, G, H>(
    expr: &str,
    all_fields: &[Field],
    is_separator: F,
    format_token: G,
    format_separator: H,
) -> String
where
    F: Fn(char) -> bool,
    G: Fn(&str, &str) -> String, // (token, safe_name) -> formatted
    H: Fn(char, &str) -> String, // (separator, current_result) -> formatted
{
    let expr = expr.trim();
    let mut result = String::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        if is_separator(ch) {
            if !current_token.is_empty() {
                // Try to find a field with this name
                let formatted =
                    if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
                        let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
                        format_token(&current_token, &safe_name)
                    } else if is_numeric_literal(&current_token) {
                        // It's a numeric literal (integer, float, or hex)
                        current_token.clone()
                    } else {
                        // Unknown token - keep as-is but make it snake_case
                        let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
                        format_token(&current_token, &safe_name)
                    };
                result.push_str(&formatted);
                current_token.clear();
            }

            result.push_str(&format_separator(ch, &result));
        } else {
            current_token.push(ch);
        }
    }

    // Handle any remaining token
    if !current_token.is_empty() {
        let formatted = if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
            let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
            format_token(&current_token, &safe_name)
        } else if is_numeric_literal(&current_token) {
            current_token.clone()
        } else {
            let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
            format_token(&current_token, &safe_name)
        };
        result.push_str(&formatted);
    }

    result.trim().to_string()
}

/// Convert a condition expression from XML format to Rust code
/// Examples: "RecordCount > 0" -> "record_count > 0"
pub fn convert_condition_expression(expr: &str, all_fields: &[Field]) -> String {
    parse_expression(
        expr,
        all_fields,
        // Separator check: comparison and logical operators
        |ch| {
            ch.is_whitespace()
                || ch == '>'
                || ch == '<'
                || ch == '='
                || ch == '!'
                || ch == '&'
                || ch == '|'
                || ch == '('
                || ch == ')'
        },
        // Token formatting: just use the safe name as-is
        |_token, safe_name| safe_name.to_string(),
        // Separator formatting: preserve non-whitespace, normalize whitespace
        |ch, result| {
            if !ch.is_whitespace() {
                ch.to_string()
            } else if !result.is_empty() && !result.ends_with(' ') {
                " ".to_string()
            } else {
                String::new()
            }
        },
    )
}

/// Convert a length expression from XML format to Rust code
/// Examples: "Count" -> "count", "RecordCount - 1" -> "(record_count - 1)"
pub fn convert_length_expression(expr: &str, all_fields: &[Field]) -> String {
    let expr = expr.trim();

    // Special case: "*" means read remaining bytes - this will be handled specially
    if expr == "*" {
        return "*".to_string();
    }

    // Check if it's a simple field reference
    if let Some(field) = all_fields.iter().find(|f| f.name == expr) {
        let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
        return format!("{} as usize", safe_name);
    }

    parse_expression(
        expr,
        all_fields,
        // Separator check: arithmetic operators
        |ch| {
            ch.is_whitespace()
                || ch == '+'
                || ch == '-'
                || ch == '*'
                || ch == '/'
                || ch == '('
                || ch == ')'
        },
        // Token formatting: wrap field names in "as usize"
        |_token, safe_name| format!("{} as usize", safe_name),
        // Separator formatting: add spaces around operators
        |ch, _result| {
            if !ch.is_whitespace() {
                format!(" {} ", ch)
            } else {
                String::new()
            }
        },
    )
}
