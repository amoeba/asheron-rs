use crate::{
    identifiers::{IdentifierType, safe_identifier},
    types::Field,
};

/// Convert a condition expression from XML format to Rust code for writers
/// This is similar to the reader version but adds `self.` prefix to field references
/// Examples: "RecordCount > 0" -> "self.record_count > 0"
pub fn convert_condition_expression(expr: &str, all_fields: &[Field]) -> String {
    crate::generation::read_generation::expression_readers::parse_expression(
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
        // Token formatting: add `self.` prefix to field names
        |_token, safe_name| format!("self.{}", safe_name),
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

/// Convert a length expression from XML format to Rust code for writers
/// Examples: "Count" -> "self.count", "RecordCount - 1" -> "(self.record_count - 1)"
pub fn convert_length_expression(expr: &str, all_fields: &[Field]) -> String {
    let expr = expr.trim();

    // Special case: "*" means write all bytes - this will be handled specially
    if expr == "*" {
        return "*".to_string();
    }

    // Check if it's a simple field reference
    if let Some(field) = all_fields.iter().find(|f| f.name == expr) {
        let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
        return format!("self.{} as usize", safe_name);
    }

    crate::generation::read_generation::expression_readers::parse_expression(
        expr,
        all_fields,
        // Separator check: arithmetic and logical operators, parentheses
        |ch| {
            ch.is_whitespace()
                || ch == '+'
                || ch == '-'
                || ch == '*'
                || ch == '/'
                || ch == '('
                || ch == ')'
        },
        // Token formatting: add `self.` prefix
        |_token, safe_name| format!("self.{}", safe_name),
        // Separator formatting
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
