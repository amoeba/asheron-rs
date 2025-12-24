use crate::{
    identifiers::{IdentifierType, safe_identifier},
    type_utils::get_rust_type,
    types::Field,
};

use super::expression_writers::convert_condition_expression;
use crate::generation::context::ReaderContext;

/// Generate the appropriate write call for a given field
///
/// # Arguments
/// * `ctx` - Reader context with enum information (reused for writers)
/// * `field` - The field to generate a write call for
/// * `all_fields` - All fields available in the current scope (used to resolve length expressions)
pub fn generate_write_call(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
    let base_write = generate_write_base_logic(ctx, field, &field_name, all_fields);

    if field.is_optional {
        // For optional fields, need to handle dereferencing for primitives
        let value_expr = get_value_expr_for_optional(&field.field_type);
        // Replace both `&self.field` and `self.field` patterns to handle collection types
        let write_with_value = base_write
            .replace(&format!("&self.{}", field_name), &value_expr)
            .replace(&format!("self.{}", field_name), &value_expr);

        // Generate conditional write based on the test condition or mask
        if let Some(condition) = &field.optional_condition {
            // <if test="..."> condition
            // Convert the condition to Rust code
            let rust_condition = convert_condition_expression(condition, all_fields);
            // Only write if condition is true and field is Some (using let-chain)
            format!(
                "if {}\n            && let Some(ref value) = self.{} {{\n                {};\n            }}",
                rust_condition, field_name, write_with_value
            )
        } else if let (Some(mask_field), Some(mask_value)) = (&field.mask_field, &field.mask_value)
        {
            // <maskmap> with <mask value="...">
            // Generate bitwise AND check
            let mask_field_safe = safe_identifier(mask_field, IdentifierType::Field).name;
            // The mask_value might be a plain hex number or an enum variant like "ACBaseQualitiesFlags.PropertyInt"
            let mask_value_code = if mask_value.contains('.') {
                // It's an enum variant reference, convert to Rust format
                let parts: Vec<&str> = mask_value.split('.').collect();
                if parts.len() == 2 {
                    let enum_name = parts[0];
                    let variant_name = parts[1];
                    // Check if this is a mask enum (bitflags)
                    if ctx.mask_enums.contains(enum_name) {
                        // Convert variant name to SCREAMING_SNAKE_CASE for bitflags
                        let const_name = if let Some(stripped) = variant_name.strip_prefix("0x") {
                            format!("TYPE_{}", stripped.to_uppercase())
                        } else {
                            // Convert from PascalCase to SCREAMING_SNAKE_CASE
                            let mut result = String::new();
                            let mut prev_was_lower = false;
                            for (i, ch) in variant_name.chars().enumerate() {
                                if ch.is_uppercase() && i > 0 && prev_was_lower {
                                    result.push('_');
                                }
                                result.push(ch.to_ascii_uppercase());
                                prev_was_lower = ch.is_lowercase();
                            }
                            result
                        };
                        format!("{}::{}.bits()", enum_name, const_name)
                    } else {
                        // Regular enum - cast to u32
                        format!("{}::{} as u32", enum_name, variant_name)
                    }
                } else {
                    mask_value.clone()
                }
            } else {
                mask_value.clone()
            };

            // Cast mask field to u32 if it's an enum type
            let mask_field_expr =
                generate_mask_field_expr(ctx, mask_field, &mask_field_safe, all_fields);

            // Generate: if (flags.bits() & mask) != 0 && let Some(ref value) = field { write(value) }
            format!(
                "if ({} & {}) != 0\n            && let Some(ref value) = self.{} {{\n                {};\n            }}",
                mask_field_expr, mask_value_code, field_name, write_with_value
            )
        } else {
            // No condition - only write if Some
            format!(
                "if let Some(ref value) = self.{} {{\n            {};\n        }}",
                field_name, write_with_value
            )
        }
    } else {
        base_write
    }
}

/// Get the value expression for an optional field
/// For Copy types (primitives), we need to dereference (*value)
/// For non-Copy types (custom types), we keep it as a reference (value)
fn get_value_expr_for_optional(field_type: &str) -> String {
    let rust_type = get_rust_type(field_type);

    // Primitive types that need dereferencing
    match rust_type {
        "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "f32" | "f64" | "bool" => {
            "*value".to_string()
        }
        _ => {
            // For complex types, use the reference as-is
            "value".to_string()
        }
    }
}

/// Generate a write call for a field that is already within a conditional context
/// This is used when generating code inside an if block where the condition is already checked
pub fn generate_conditional_write_call(
    ctx: &ReaderContext,
    field: &Field,
    all_fields: &[Field],
) -> String {
    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
    let base_write = generate_write_base_logic(ctx, field, &field_name, all_fields);

    // For optional fields in conditional context, unwrap the Option
    if field.is_optional {
        let value_expr = get_value_expr_for_optional(&field.field_type);
        let write_with_value = base_write.replace(&format!("self.{}", field_name), &value_expr);

        format!(
            "if let Some(ref value) = self.{} {{\n                {};\n            }}",
            field_name, write_with_value
        )
    } else {
        base_write
    }
}

/// Helper function that generates base write call without optional field handling
pub fn generate_write_base_logic(
    ctx: &ReaderContext,
    field: &Field,
    field_name: &str,
    _all_fields: &[Field],
) -> String {
    let field_type = &field.field_type;

    // Handle alignment padding fields first
    if let Some(align_type) = field_type.strip_prefix("__align__") {
        let padding_code = match align_type {
            "DWORD" => "align_dword_write(writer)?",
            "QWORD" => "align_qword_write(writer)?",
            _ => "Ok::<(), Box<dyn std::error::Error>>(())?",
        };
        return padding_code.to_string();
    }

    let rust_type = get_rust_type(field_type);

    match rust_type {
        "u8" => format!("write_u8(writer, self.{})?", field_name),
        "i8" => format!("write_i8(writer, self.{})?", field_name),
        "u16" => format!("write_u16(writer, self.{})?", field_name),
        "i16" => format!("write_i16(writer, self.{})?", field_name),
        "u32" => format!("write_u32(writer, self.{})?", field_name),
        "i32" => format!("write_i32(writer, self.{})?", field_name),
        "u64" => format!("write_u64(writer, self.{})?", field_name),
        "i64" => format!("write_i64(writer, self.{})?", field_name),
        "f32" => format!("write_f32(writer, self.{})?", field_name),
        "f64" => format!("write_f64(writer, self.{})?", field_name),
        "bool" => format!("write_bool(writer, self.{})?", field_name),
        "String" => format!("write_string(writer, &self.{})?", field_name),
        "WString" => format!("write_wstring(writer, &self.{}.0)?", field_name),
        _ => {
            // Check if it's an enum
            if ctx.enum_parent_map.contains_key(field_type) {
                // It's an enum - convert to parent type and write
                if let Some(parent_type) = ctx.enum_parent_map.get(field_type) {
                    let parent_rust_type = get_rust_type(parent_type);
                    let write_fn =
                        crate::generation::helpers::get_writer_function_name(parent_rust_type)
                            .unwrap_or_else(|| {
                                panic!("Unsupported enum parent type: {}", parent_type)
                            });

                    // Use .bits() for bitflags, as u32 for regular enums
                    if ctx.mask_enums.contains(field_type) {
                        format!("{}(writer, self.{}.bits())?", write_fn, field_name)
                    } else {
                        format!(
                            "{}(writer, self.{}.clone() as {})?",
                            write_fn, field_name, parent_rust_type
                        )
                    }
                } else {
                    format!("self.{}.write(writer)?", field_name)
                }
            } else if field_type == "DataId" && field.param.is_some() {
                // DataId with param attribute should be written as PackedDWORD (variable-length)
                format!("write_packed_dword(writer, self.{}.0)?", field_name)
            } else if field_type.starts_with("Vec<") {
                // Handle Vec - extract element type
                let element_type = &field_type[4..field_type.len() - 1];

                if field.length_expression.is_some() {
                    // Vec with explicit length expression - just write the elements
                    super::collection_writers::generate_vec_write(element_type, field_name)
                } else {
                    "unimplemented!(\"Vec writing without length not yet implemented\")".to_string()
                }
            } else if field_type.starts_with("PackableList<") {
                let element_type = &field_type[13..field_type.len() - 1];
                super::collection_writers::generate_packable_list_write(element_type, field_name)
            } else if field_type.starts_with("std::collections::HashMap<") {
                // Handle HashMap - extract key and value types
                let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();

                    if field.length_expression.is_some() {
                        super::collection_writers::generate_hashmap_write(
                            key_type, value_type, field_name, ctx,
                        )
                    } else {
                        "unimplemented!(\"HashMap writing without length not yet implemented\")"
                            .to_string()
                    }
                } else {
                    "unimplemented!(\"HashMap writing with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PackableHashTable<") {
                let inner = &field_type["PackableHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    super::collection_writers::generate_packable_hash_table_write(
                        key_type, value_type, field_name, ctx,
                    )
                } else {
                    "unimplemented!(\"PackableHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PHashTable<") {
                let inner = &field_type["PHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    super::collection_writers::generate_phash_table_write(
                        key_type, value_type, field_name,
                    )
                } else {
                    "unimplemented!(\"PHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else {
                // Custom struct type - call its write method
                format!("self.{}.write(writer)?", field_name)
            }
        }
    }
}

/// Generate mask field expression, casting to u32 if it's an enum type
fn generate_mask_field_expr(
    ctx: &ReaderContext,
    mask_field_name: &str,
    mask_field_safe: &str,
    all_fields: &[Field],
) -> String {
    if let Some(mask_field_obj) = all_fields.iter().find(|f| f.name == mask_field_name) {
        let base_expr = if ctx.enum_parent_map.contains_key(&mask_field_obj.field_type) {
            // It's an enum or bitflags - get bits representation
            if ctx.mask_enums.contains(&mask_field_obj.field_type) {
                // Bitflags - use .bits() method
                format!("self.{}.bits()", mask_field_safe)
            } else {
                // Regular enum - clone and cast to u32 (enums don't derive Copy)
                format!("self.{}.clone() as u32", mask_field_safe)
            }
        } else {
            format!("self.{}", mask_field_safe)
        };
        // If the mask field is optional, unwrap it with a default of 0
        if mask_field_obj.is_optional {
            format!("{}.unwrap_or(0)", base_expr)
        } else {
            base_expr
        }
    } else {
        format!("self.{}", mask_field_safe)
    }
}
