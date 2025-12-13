use std::collections::BTreeMap;

use crate::{
    field_gen::{DEFAULT_ENUM_DERIVES, build_derive_string},
    identifiers::{safe_enum_variant_name, to_snake_case},
    type_utils::get_rust_type,
    types::ProtocolEnum,
    util::format_hex_value,
};

pub fn generate_bitflags(protocol_enum: &ProtocolEnum) -> String {
    let enum_name = &protocol_enum.name;
    let mut out = String::new();

    if let Some(text_str) = &protocol_enum.text {
        out.push_str(&format!("/// {text_str}\n"));
    }

    // Get the underlying type for the bitflags
    let repr_type = if !protocol_enum.parent.is_empty() {
        get_rust_type(&protocol_enum.parent)
    } else {
        "u32" // Default to u32 if no parent specified
    };

    // Generate bitflags! macro invocation
    out.push_str("bitflags::bitflags! {\n");
    out.push_str("    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]\n");
    out.push_str(&format!("    pub struct {}: {} {{\n", enum_name, repr_type));

    for enum_value in &protocol_enum.values {
        let original_name = &enum_value.name;

        // Generate constant name - convert to SCREAMING_SNAKE_CASE for bitflags constants
        let const_name = if let Some(stripped) = original_name.strip_prefix("0x") {
            format!("TYPE_{}", stripped.to_uppercase())
        } else {
            // Convert from PascalCase to SCREAMING_SNAKE_CASE
            let mut result = String::new();
            let mut prev_was_lower = false;
            for (i, ch) in original_name.chars().enumerate() {
                if ch.is_uppercase() && i > 0 && prev_was_lower {
                    result.push('_');
                }
                result.push(ch.to_ascii_uppercase());
                prev_was_lower = ch.is_lowercase();
            }
            result
        };

        // Format the value as hex literal
        let value_literal = if enum_value.value < 0 {
            format!("{}", enum_value.value)
        } else {
            format!("0x{:X}", enum_value.value)
        };

        out.push_str(&format!(
            "        const {} = {};\n",
            const_name, value_literal
        ));
    }

    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACDataType implementation for bitflags
    if !protocol_enum.parent.is_empty() {
        out.push_str(&super::helpers::generate_acdata_type_impl(
            enum_name,
            &protocol_enum.parent,
            "from_bits_retain",
        ));
    }

    out
}

pub fn generate_message_queue_enum(
    c2s_types: &[crate::types::ProtocolType],
    s2c_types: &[crate::types::ProtocolType],
    game_action_types: &[crate::types::ProtocolType],
    game_event_types: &[crate::types::ProtocolType],
) -> String {
    let mut out = String::new();

    // Generate MessageQueue enum
    out.push_str("/// Message queue types from protocol.xml\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]\n");
    out.push_str("pub enum MessageQueue {\n");

    // Collect all unique queue values
    let mut queues = std::collections::HashSet::new();
    for t in c2s_types
        .iter()
        .chain(s2c_types.iter())
        .chain(game_action_types.iter())
        .chain(game_event_types.iter())
    {
        if let Some(ref queue) = t.queue {
            queues.insert(queue.clone());
        }
    }

    // Sort for deterministic output
    let mut queue_list: Vec<_> = queues.into_iter().collect();
    queue_list.sort();

    for queue in &queue_list {
        // Convert queue name to valid Rust identifier
        let snake_case = to_snake_case(queue);
        let pascal_case = snake_case
            .split('_')
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        out.push_str(&format!("    {},\n", pascal_case));
    }

    out.push_str("}\n\n");
    out
}

pub fn generate_enum(protocol_enum: &ProtocolEnum) -> String {
    let enum_name = &protocol_enum.name;
    let mut out = String::new();

    if let Some(text_str) = &protocol_enum.text {
        out.push_str(&format!("/// {text_str}\n"));
    }

    // Generate all enums as regular enums (including mask enums)
    // Use enum-specific derives that include TryFromPrimitive
    let mut all_derives: Vec<String> = DEFAULT_ENUM_DERIVES.iter().map(|s| s.to_string()).collect();
    all_derives.extend(protocol_enum.extra_derives.iter().cloned());
    let derives = format!("#[derive({})]", all_derives.join(", "));

    // Add repr if parent is specified
    let repr_attr = if !protocol_enum.parent.is_empty() {
        let repr_type = get_rust_type(&protocol_enum.parent);
        format!("#[repr({repr_type})]\n")
    } else {
        String::new()
    };

    out.push_str(&format!("{repr_attr}{derives}\npub enum "));
    out.push_str(enum_name);
    out.push_str(" {\n");

    for enum_value in &protocol_enum.values {
        let original_name = &enum_value.name;

        // Generate variant name - convert to PascalCase if it has underscores
        let variant_name = if let Some(stripped) = original_name.strip_prefix("0x") {
            format!("Type{stripped}")
        } else {
            original_name.clone()
        };

        // Check if variant name is a reserved word or needs conversion
        let safe_variant = safe_enum_variant_name(&variant_name);

        // Determine if we need serde rename (if the safe name differs from original)
        let needs_serde_rename = safe_variant.name != *original_name;

        // Format the value as hex literal for enum definition
        let value_literal = if enum_value.value < 0 {
            // Negative values use decimal
            format!("{}", enum_value.value)
        } else {
            // Positive values use hex literal (natural width, no padding)
            format!("0x{:X}", enum_value.value)
        };

        if needs_serde_rename {
            out.push_str(&format!(
                "    #[serde(rename = \"{}\")]\n    {} = {},\n",
                original_name, safe_variant.name, value_literal
            ));
        } else {
            out.push_str(&format!("    {} = {},\n", safe_variant.name, value_literal));
        }
    }

    out.push_str("}\n\n");

    // Add ACDataType implementation for enums
    if !protocol_enum.parent.is_empty() {
        out.push_str(&super::helpers::generate_acdata_type_impl(
            enum_name,
            &protocol_enum.parent,
            "try_from",
        ));
    }

    out
}

/// Generate a name for a variant struct
pub fn generate_variant_struct_name(parent_type_name: &str, case_value: i64) -> String {
    let case_hex_str = if case_value < 0 {
        format!("Neg{}", case_value.abs())
    } else {
        format!("{:X}", case_value)
    };
    let name = format!("{parent_type_name}Type{case_hex_str}");
    // Apply safe_identifier to ensure proper Rust PascalCase naming
    let safe_name =
        crate::identifiers::safe_identifier(&name, crate::identifiers::IdentifierType::Type);
    safe_name.name
}

/// Generate a standalone struct for a single enum variant with a nested switch
pub fn generate_variant_struct(
    parent_type_name: &str,
    case_value: i64,
    field_set: &crate::types::FieldSet,
    switch_field: &str,
    case_fields: &[crate::types::Field],
) -> String {
    let struct_name = generate_variant_struct_name(parent_type_name, case_value);
    let mut out = String::new();

    let derives = build_derive_string(&[]);
    out.push_str(&format!("{derives}\n"));
    out.push_str(&format!("pub struct {struct_name} {{\n"));

    // Check if this case has a nested switch
    let has_nested_switch = if let Some(ref nested_switches) = field_set.nested_switches {
        nested_switches.contains_key(&case_value)
    } else {
        false
    };

    // Get the nested switch field name to skip it in common fields
    let nested_switch_field_name = if has_nested_switch {
        field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .map(|ns| ns.switch_field.clone())
    } else {
        None
    };

    // Add common fields first (excluding the switch fields)
    for field in &field_set.common_fields {
        // Skip the outer switch field, nested switch field, and alignment marker fields
        if field.name != switch_field
            && Some(field.name.clone()) != nested_switch_field_name
            && !field.name.starts_with("__alignment_marker_")
        {
            out.push_str(&crate::field_gen::generate_field_line(field, false)); // false = struct field
            out.push_str(",\n");
        }
    }

    // Add variant-specific fields before the nested switch
    // We already have nested_switch_field_name from above
    for field in case_fields {
        // Skip the nested switch discriminator field - it will be represented by the enum
        // Also skip alignment marker fields - they're only for reading
        let skip_field = nested_switch_field_name.as_ref() == Some(&field.name);
        if !skip_field && !field.name.starts_with("__alignment_marker_") {
            out.push_str(&crate::field_gen::generate_field_line(field, false));
            out.push_str(",\n");
        }
    }

    if has_nested_switch {
        let nested_switch_obj = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();

        // Add common fields for the nested switch (fields that come after case fields but before the switch cases)
        // Skip the switch field itself since it becomes the nested enum
        for field in &nested_switch_obj.common_fields {
            if field.name != nested_switch_obj.switch_field {
                out.push_str(&crate::field_gen::generate_field_line(field, false));
                out.push_str(",\n");
            }
        }

        // Generate a nested enum for the switch
        let nested_enum_name_raw = format!(
            "{struct_name}{}{}",
            nested_switch_obj.switch_field, "Variant"
        );
        let nested_enum_name = crate::identifiers::safe_identifier(
            &nested_enum_name_raw,
            crate::identifiers::IdentifierType::Type,
        )
        .name;
        out.push_str(&format!(
            "    pub {}: {nested_enum_name},\n",
            crate::identifiers::safe_identifier(
                &nested_switch_obj.switch_field,
                crate::identifiers::IdentifierType::Field
            )
            .name
        ));

        // Add trailing fields (fields after the nested switch within the case)
        for field in &nested_switch_obj.trailing_fields {
            out.push_str(&crate::field_gen::generate_field_line(field, false));
            out.push_str(",\n");
        }
    }

    out.push_str("}\n\n");

    // If there's a nested switch, generate the nested enum
    if has_nested_switch {
        let nested_switch_obj = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();
        let nested_enum_name_raw = format!(
            "{struct_name}{}{}",
            nested_switch_obj.switch_field, "Variant"
        );
        let nested_enum_name = crate::identifiers::safe_identifier(
            &nested_enum_name_raw,
            crate::identifiers::IdentifierType::Type,
        )
        .name;

        out.push_str(&generate_nested_switch_enum(
            &nested_enum_name,
            nested_switch_obj,
        ));
    }

    out
}

/// Generate an enum for a nested switch
pub fn generate_nested_switch_enum(
    enum_name: &str,
    nested_switch: &crate::types::NestedSwitch,
) -> String {
    let mut out = String::new();

    // Group nested case values by field signature
    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

    for (case_value, case_fields) in &nested_switch.variant_fields {
        let field_sig = case_fields
            .iter()
            .map(|f| format!("{}:{}", f.name, f.field_type))
            .collect::<Vec<_>>()
            .join(";");

        field_groups
            .entry(field_sig)
            .or_insert_with(|| (*case_value, Vec::new()))
            .1
            .push(*case_value);
    }

    // Sort by primary value for consistent output
    let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    // First, generate standalone variant structs
    for (_field_sig, (_primary_value, all_values)) in &sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();
        let first_value = sorted_values[0];

        let variant_name = super::helpers::generate_variant_name(first_value);

        let struct_name = format!("{}{}", enum_name, variant_name);

        let derives = build_derive_string(&[]);
        out.push_str(&format!("{derives}\n"));
        out.push_str(&format!("pub struct {struct_name} {{\n"));

        if let Some(case_fields) = nested_switch.variant_fields.get(&first_value) {
            for field in case_fields {
                out.push_str(&crate::field_gen::generate_field_line(field, false)); // false = is struct field
                out.push_str(",\n");
            }
        }

        out.push_str("}\n\n");
    }

    // Now generate the enum that references the standalone structs
    let derives = build_derive_string(&[]);
    out.push_str(&format!("{derives}\n"));
    out.push_str("#[serde(tag = \"");
    out.push_str(&nested_switch.switch_field);
    out.push_str("\")]\n");
    out.push_str(&format!("pub enum {enum_name} {{\n"));

    for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
        all_values.sort();

        let first_value = all_values[0];
        let first_value_str = format_hex_value(first_value);

        let variant_name = super::helpers::generate_variant_name(first_value);

        let struct_name = format!("{}{}", enum_name, variant_name);

        out.push_str(&format!("    #[serde(rename = \"{first_value_str}\")]\n"));

        for alias_value in &all_values[1..] {
            let alias_str = format_hex_value(*alias_value);
            out.push_str(&format!("    #[serde(alias = \"{alias_str}\")]\n"));
        }

        out.push_str(&format!("    {variant_name}({struct_name}),\n"));
    }

    out.push_str("}\n\n");
    out
}
