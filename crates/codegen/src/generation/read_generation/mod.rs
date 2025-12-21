pub mod collection_readers;
pub mod expression_readers;
pub mod primitive_readers;

// Re-export the functions needed by other modules
pub use collection_readers::{
    generate_hashmap_read_with_length, generate_packable_hash_table_read,
    generate_packable_list_read, generate_phash_table_read, generate_vec_read_with_length,
};
pub use expression_readers::convert_length_expression;
pub use primitive_readers::{
    convert_condition_expression, generate_conditional_read_call, generate_read_base_logic,
    generate_read_call,
};

use crate::{
    field_gen::get_allow_unused_directive,
    generation::context::ReaderContext,
    identifiers::{IdentifierType, safe_identifier},
    type_utils::get_rust_type,
    types::{Field, IfBranch},
};

/// A group of consecutive fields with the same condition
#[derive(Debug)]
pub struct FieldGroup<'a> {
    pub condition: ConditionKey,
    pub fields: Vec<&'a Field>,
}

/// Key for grouping fields by their condition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConditionKey {
    /// No condition - always read
    None,
    /// <if test="condition">
    IfTest(String),
    /// <maskmap> with specific field and mask value
    Mask { field: String, value: String },
}

impl ConditionKey {
    pub fn from_field(field: &Field) -> Self {
        if let Some(condition) = &field.optional_condition {
            ConditionKey::IfTest(condition.clone())
        } else if let (Some(mask_field), Some(mask_value)) = (&field.mask_field, &field.mask_value)
        {
            ConditionKey::Mask {
                field: mask_field.clone(),
                value: mask_value.clone(),
            }
        } else {
            ConditionKey::None
        }
    }
}

/// Groups consecutive fields by their condition to reduce repetitive conditional checks
pub fn group_consecutive_fields_by_condition(fields: &[Field]) -> Vec<FieldGroup<'_>> {
    let mut groups = Vec::new();

    if fields.is_empty() {
        return groups;
    }

    let mut current_key = ConditionKey::from_field(&fields[0]);
    let mut current_group = vec![&fields[0]];

    for field in &fields[1..] {
        let key = ConditionKey::from_field(field);

        if key == current_key {
            // Same condition - add to current group
            current_group.push(field);
        } else {
            // Different condition - start new group
            groups.push(FieldGroup {
                condition: current_key,
                fields: current_group,
            });
            current_key = key;
            current_group = vec![field];
        }
    }

    // Don't forget the last group
    groups.push(FieldGroup {
        condition: current_key,
        fields: current_group,
    });

    groups
}

/// Generate reads for a group of fields with the same condition
pub fn generate_field_group_reads(
    ctx: &ReaderContext,
    type_name: &str,
    out: &mut String,
    condition_key: &ConditionKey,
    fields: &[&Field],
    all_fields: &[Field],
) {
    match condition_key {
        ConditionKey::None => {
            // No condition - just read each field directly
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                let read_call = primitive_readers::generate_read_call(ctx, field, all_fields);
                let allow_directive = get_allow_unused_directive(type_name, &field_name);
                out.push_str(allow_directive);

                // Add field-level tracing span
                out.push_str("        #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("        let _field_span_{} = {{\n", field_name));
                out.push_str("            let pos = reader.stream_position().unwrap_or(0);\n");
                out.push_str(&format!("            tracing::span!(tracing::Level::TRACE, \"field\", name = \"{}\", position = pos).entered()\n", field.name));
                out.push_str("        };\n");

                // Alignment fields don't need to be stored, just executed
                if field.name.starts_with("__alignment_marker_") {
                    out.push_str(&format!("        {}?;\n", read_call));
                } else {
                    out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));
                }

                // Drop field span
                out.push_str("        #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("        drop(_field_span_{});\n", field_name));

                // Generate subfield computations if any
                for subfield in &field.subfields {
                    let subfield_name = safe_identifier(&subfield.name, IdentifierType::Field).name;
                    let allow_directive = get_allow_unused_directive(type_name, &subfield_name);
                    let subfield_expr = expression_readers::convert_condition_expression(
                        &subfield.value_expression,
                        all_fields,
                    );
                    let subfield_rust_type = get_rust_type(&subfield.field_type);
                    out.push_str(allow_directive);
                    out.push_str(&format!(
                        "        let {} = ({}) as {};\n",
                        subfield_name, subfield_expr, subfield_rust_type
                    ));
                }
            }
        }
        ConditionKey::IfTest(condition) => {
            // Handle <if test="condition"> with <true> and <false> branches
            let rust_condition =
                expression_readers::convert_condition_expression(condition, all_fields);

            // Categorize fields by which branch(es) they belong to
            let mut true_only = Vec::new();
            let mut false_only = Vec::new();
            let mut both = Vec::new();

            for field in fields {
                match &field.if_branch {
                    Some(IfBranch::True) => true_only.push(*field),
                    Some(IfBranch::False) => false_only.push(*field),
                    Some(IfBranch::Both) => both.push(*field),
                    None => both.push(*field), // Fallback for fields without explicit branch
                }
            }

            // Declare fields that need to be available after the if-else block.
            // - true_only and false_only: initialized to None, may or may not be assigned
            // - both: will be assigned in both branches, declared with underscore init (never read)
            for field in true_only.iter().chain(false_only.iter()) {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let mut {} = None;\n", field_name));
            }
            for field in &both {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let {};\n", field_name));
            }

            // Generate if-else block that assigns values to the variables
            out.push_str(&format!("        if {} {{\n", rust_condition));

            // TRUE branch: read all true-only and both-branch fields
            for field in &true_only {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                // Add field-level tracing span
                out.push_str("            #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("            let _field_span_{} = {{\n", field_name));
                out.push_str("                let pos = reader.stream_position().unwrap_or(0);\n");
                out.push_str(&format!("                tracing::span!(tracing::Level::TRACE, \"field\", name = \"{}\", position = pos).entered()\n", field.name));
                out.push_str("            };\n");
                // For conditional fields, generate_read_call already handles optionality
                // by returning an expression that evaluates to Option<T> based on the condition
                out.push_str(&format!(
                    "            {} = {}?;\n",
                    field_name,
                    primitive_readers::generate_read_call(ctx, field, all_fields)
                ));
                // Drop field span
                out.push_str("            #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("            drop(_field_span_{});\n", field_name));
            }
            for field in &both {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                // Add field-level tracing span
                out.push_str("            #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("            let _field_span_{} = {{\n", field_name));
                out.push_str("                let pos = reader.stream_position().unwrap_or(0);\n");
                out.push_str(&format!("                tracing::span!(tracing::Level::TRACE, \"field\", name = \"{}\", position = pos).entered()\n", field.name));
                out.push_str("            };\n");
                // For fields that exist in both branches, read directly without wrapping in Some
                // since the struct field is not Option<T> for these (they're always present)
                out.push_str(&format!(
                    "            {} = {}?;\n",
                    field_name,
                    primitive_readers::generate_read_call(ctx, field, all_fields)
                ));
                // Drop field span
                out.push_str("            #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("            drop(_field_span_{});\n", field_name));
            }

            // FALSE branch: only emit if there are fields to read in the false branch
            if !false_only.is_empty() || !both.is_empty() {
                out.push_str("        } else {\n");

                // Read false-only fields
                for field in &false_only {
                    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    // Add field-level tracing span
                    out.push_str("            #[cfg(feature = \"tracing\")]\n");
                    out.push_str(&format!("            let _field_span_{} = {{\n", field_name));
                    out.push_str("                let pos = reader.stream_position().unwrap_or(0);\n");
                    out.push_str(&format!("                tracing::span!(tracing::Level::TRACE, \"field\", name = \"{}\", position = pos).entered()\n", field.name));
                    out.push_str("            };\n");
                    // For conditional fields, generate_read_call already handles optionality
                    // by returning an expression that evaluates to Option<T> based on the condition
                    out.push_str(&format!(
                        "            {} = {}?;\n",
                        field_name,
                        primitive_readers::generate_read_call(ctx, field, all_fields)
                    ));
                    // Drop field span
                    out.push_str("            #[cfg(feature = \"tracing\")]\n");
                    out.push_str(&format!("            drop(_field_span_{});\n", field_name));
                }

                // Read both-branch fields, with type casting if needed
                for field in &both {
                    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    // Add field-level tracing span
                    out.push_str("            #[cfg(feature = \"tracing\")]\n");
                    out.push_str(&format!("            let _field_span_{} = {{\n", field_name));
                    out.push_str("                let pos = reader.stream_position().unwrap_or(0);\n");
                    out.push_str(&format!("                tracing::span!(tracing::Level::TRACE, \"field\", name = \"{}\", position = pos).entered()\n", field.name));
                    out.push_str("            };\n");
                    // For fields that exist in both branches, read directly without wrapping in Some
                    // since the struct field is not Option<T> for these (they're always present)
                    let read_call = primitive_readers::generate_read_call(ctx, field, all_fields);
                    out.push_str(&format!("            {} = {}?;\n", field_name, read_call));
                    // Drop field span
                    out.push_str("            #[cfg(feature = \"tracing\")]\n");
                    out.push_str(&format!("            drop(_field_span_{});\n", field_name));
                }

                out.push_str("        }\n");
            } else {
                out.push_str("        }\n");
            }
        }
        ConditionKey::Mask {
            field: mask_field,
            value: mask_value,
        } => {
            // Initialize all optional fields to None first
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let mut {} = None;\n", field_name));
            }

            // Generate the if block
            let mask_field_safe = safe_identifier(mask_field, IdentifierType::Field).name;
            let mask_value_code = if mask_value.contains('.') {
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

            out.push_str(&format!(
                "        if ({} & {}) != 0 {{\n",
                mask_field_expr, mask_value_code
            ));
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                // Add field-level tracing span
                out.push_str("            #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("            let _field_span_{} = {{\n", field_name));
                out.push_str("                let pos = reader.stream_position().unwrap_or(0);\n");
                out.push_str(&format!("                tracing::span!(tracing::Level::TRACE, \"field\", name = \"{}\", position = pos).entered()\n", field.name));
                out.push_str("            };\n");
                // For maskmap fields: we read the raw value and wrap it in Some(),
                // since the variable was initialized as None (making it Option<T>)
                let read_call = primitive_readers::generate_read_base_logic(ctx, field, all_fields);
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name, read_call
                ));
                // Drop field span
                out.push_str("            #[cfg(feature = \"tracing\")]\n");
                out.push_str(&format!("            drop(_field_span_{});\n", field_name));
            }
            out.push_str("        }\n");
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
