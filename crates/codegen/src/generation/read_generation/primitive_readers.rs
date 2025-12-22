use crate::{
    identifiers::{IdentifierType, safe_identifier},
    type_utils::get_rust_type,
    types::Field,
};

use super::expression_readers::convert_condition_expression;
use crate::generation::context::ReaderContext;

/// Generate the appropriate read call for a given field
///
/// # Arguments
/// * `ctx` - Reader context with enum information
/// * `field` - The field to generate a read call for
/// * `all_fields` - All fields available in the current scope (used to resolve length expressions)
pub fn generate_read_call(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let base_read = generate_read_base_logic(ctx, field, all_fields);

    if field.is_optional {
        // Generate conditional read based on the test condition or mask
        if let Some(condition) = &field.optional_condition {
            // <if test="..."> condition
            // Convert the condition to Rust code (e.g., "RecordCount > 0" -> "record_count > 0")
            let rust_condition = convert_condition_expression(condition, all_fields);
            // The base_read returns Result<T, E>, we want Result<Option<T>, E>
            // If condition is true, read and wrap in Some; otherwise return Ok(None)
            format!(
                "if {} {{ {}.map(Some) }} else {{ Ok(None) }}",
                rust_condition, base_read
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

            // Generate: if (flags.clone() as u32 & 0x8) != 0 { read().map(Some) } else { Ok(None) }
            format!(
                "if ({} & {}) != 0 {{ {}.map(Some) }} else {{ Ok(None) }}",
                mask_field_expr, mask_value_code, base_read
            )
        } else {
            // No condition - this shouldn't happen for truly optional fields, but handle it
            base_read
        }
    } else {
        base_read
    }
}

/// Generate a read call for a field that is already within a conditional context
/// This is used when generating code inside an if block where the condition is already checked
pub fn generate_conditional_read_call(
    ctx: &ReaderContext,
    field: &Field,
    all_fields: &[Field],
) -> String {
    let base_read = generate_read_base_logic(ctx, field, all_fields);

    // Simply wrap the base read in Some() for optional fields, since we're already in conditional context
    if field.is_optional {
        format!("Some({}?)", base_read)
    } else {
        base_read
    }
}

/// Helper function that generates base read call without optional field handling
pub fn generate_read_base_logic(
    ctx: &ReaderContext,
    field: &Field,
    all_fields: &[Field],
) -> String {
    let field_type = &field.field_type;

    // Handle alignment padding fields first
    if let Some(align_type) = field_type.strip_prefix("__align__") {
        let padding_code = match align_type {
            "DWORD" => "align_dword(reader)",
            "QWORD" => "align_qword(reader)",
            _ => "read_u32(reader)",
        };
        return padding_code.to_string();
    }

    let rust_type = get_rust_type(field_type);

    match rust_type {
        "u8" => "read_u8(reader)".to_string(),
        "i8" => "read_i8(reader)".to_string(),
        "u16" => "read_u16(reader)".to_string(),
        "i16" => "read_i16(reader)".to_string(),
        "u32" => "read_u32(reader)".to_string(),
        "i32" => "read_i32(reader)".to_string(),
        "u64" => "read_u64(reader)".to_string(),
        "i64" => "read_i64(reader)".to_string(),
        "f32" => "read_f32(reader)".to_string(),
        "f64" => "read_f64(reader)".to_string(),
        "bool" => "read_bool(reader)".to_string(),
        "String" => "read_string(reader)".to_string(),
        "WString" => "read_wstring(reader).map(WString)".to_string(),
        _ => {
            // Check if it's an enum
            if let Some(parent_type) = ctx.enum_parent_map.get(field_type) {
                // It's an enum - read the parent type and cast
                let parent_rust_type = get_rust_type(parent_type);
                let read_fn = super::super::helpers::get_reader_function_name(parent_rust_type)
                    .unwrap_or_else(|| panic!("Unsupported enum parent type: {}", parent_type));
                // Use from_bits_retain for bitflags types, try_from for regular enums
                if ctx.mask_enums.contains(field_type) {
                    format!(
                        "Ok::<_, Box<dyn std::error::Error>>({}::from_bits_retain({}(reader)?))",
                        field_type, read_fn
                    )
                } else {
                    format!("{}::try_from({}(reader)?)", field_type, read_fn)
                }
            } else if field_type == "DataId" && field.param.is_some() {
                // DataId with param attribute should be read as PackedDWORD (variable-length)
                // Need to wrap the u32 result in DataId() newtype
                "read_packed_dword(reader).map(DataId)".to_string()
            } else if field_type.starts_with("Vec<") {
                // Handle Vec - extract element type
                let element_type = &field_type[4..field_type.len() - 1];

                if let Some(length_expr) = &field.length_expression {
                    // Convert length expression to Rust code
                    let length_code = super::expression_readers::convert_length_expression(
                        length_expr,
                        all_fields,
                    );
                    super::collection_readers::generate_vec_read_with_length(
                        element_type,
                        &length_code,
                    )
                } else {
                    "unimplemented!(\"Vec reading without length not yet implemented\")".to_string()
                }
            } else if field_type.starts_with("PackableList<") {
                let element_type = &field_type[13..field_type.len() - 1];
                super::collection_readers::generate_packable_list_read(element_type)
            } else if field_type.starts_with("std::collections::HashMap<") {
                // Handle HashMap - extract key and value types
                let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();

                    if let Some(length_expr) = &field.length_expression {
                        // Convert length expression to Rust code
                        let length_code = super::expression_readers::convert_length_expression(
                            length_expr,
                            all_fields,
                        );
                        super::collection_readers::generate_hashmap_read_with_length(
                            key_type,
                            value_type,
                            &length_code,
                            ctx,
                        )
                    } else {
                        "unimplemented!(\"HashMap reading without length not yet implemented\")"
                            .to_string()
                    }
                } else {
                    "unimplemented!(\"HashMap reading with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PackableHashTable<") {
                let inner = &field_type["PackableHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    super::collection_readers::generate_packable_hash_table_read(
                        key_type, value_type, ctx,
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
                    super::collection_readers::generate_phash_table_read(key_type, value_type)
                } else {
                    "unimplemented!(\"PHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else {
                // Custom struct type - call its read method
                // If the type has generic parameters (contains '<'), we need turbofish syntax
                if let Some(pos) = field_type.find('<') {
                    let (type_name, generics) = field_type.split_at(pos);
                    format!("{type_name}::{generics}::read(reader)")
                } else {
                    format!("{field_type}::read(reader)")
                }
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
                format!("{}.bits()", mask_field_safe)
            } else {
                // Regular enum - clone and cast to u32 (enums don't derive Copy)
                format!("{}.clone() as u32", mask_field_safe)
            }
        } else {
            mask_field_safe.to_string()
        };
        // If the mask field is optional, unwrap it with a default of 0
        if mask_field_obj.is_optional {
            format!("{}.unwrap_or(0)", base_expr)
        } else {
            base_expr
        }
    } else {
        mask_field_safe.to_string()
    }
}
