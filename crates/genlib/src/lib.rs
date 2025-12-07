use core::panic;
use std::collections::BTreeMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    identifiers::{IdentifierType, safe_enum_variant_name, safe_identifier},
    types::{EnumValue, Field, FieldSet, ProtocolEnum, ProtocolType},
};

mod identifiers;
mod types;

/// Default derives for all generated types
const DEFAULT_DERIVES: &[&str] = &["Clone", "Debug", "PartialEq", "Serialize", "Deserialize"];

/// Map an XML type name to a Rust type name.
pub fn get_rust_type(xml_type: &str) -> &str {
    match xml_type {
        "int" => "i32",
        "uint" => "u32",
        "short" => "i16",
        "ushort" => "u16",
        "long" => "i64",
        "ulong" => "u64",
        "byte" => "u8",
        "sbyte" => "i8",
        "float" => "f32",
        "double" => "f64",
        "bool" => "bool",
        "string" | "WString" => "String",
        // Keep other types as-is (custom types like ObjectId, Vector3, etc.)
        other => other,
    }
}

/// Get the size in bits of a primitive type
fn get_type_size(xml_type: &str) -> usize {
    match xml_type {
        "byte" | "sbyte" => 8,
        "short" | "ushort" => 16,
        "int" | "uint" | "float" => 32,
        "long" | "ulong" | "double" => 64,
        _ => 0, // Unknown or non-numeric type
    }
}

/// Returns the larger of two types (for underfull reads in <if> blocks)
/// If types differ in signedness but same size, prefer the first type
fn get_larger_type(type1: &str, type2: &str) -> String {
    let size1 = get_type_size(type1);
    let size2 = get_type_size(type2);

    if size1 >= size2 {
        type1.to_string()
    } else {
        type2.to_string()
    }
}

/// Merge fields from <if> true and false branches
/// - Fields with same name but different types: use larger type, make optional
/// - Fields only in one branch: make optional
fn merge_if_fields(true_fields: Vec<Field>, false_fields: Vec<Field>) -> Vec<Field> {
    use std::collections::HashMap;

    let mut merged: HashMap<String, Field> = HashMap::new();

    // Process true branch fields
    for field in true_fields {
        merged.insert(field.name.clone(), field);
    }

    // Process false branch fields
    for false_field in false_fields {
        if let Some(true_field) = merged.get_mut(&false_field.name) {
            // Field exists in both branches
            if true_field.field_type != false_field.field_type {
                // Different types - use the larger one
                let larger_type = get_larger_type(&true_field.field_type, &false_field.field_type);
                true_field.field_type = larger_type;
            }
            // Always optional since it's conditional
            true_field.is_optional = true;
        } else {
            // Field only in false branch - add it as optional
            merged.insert(false_field.name.clone(), false_field);
        }
    }

    // Convert to Vec and sort by name for consistent output
    let mut result: Vec<Field> = merged.into_values().collect();
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

fn generate_field_line(field: &Field) -> String {
    let original_name = &field.name;
    let safe_id = safe_identifier(original_name, IdentifierType::Field);
    let mut rust_type = get_rust_type(&field.field_type).to_string();

    // Wrap in Option if the field is optional
    if field.is_optional {
        rust_type = format!("Option<{}>", rust_type);
    }

    if safe_id.needs_rename {
        format!(
            "        #[serde(rename = \"{original_name}\")]\n        {}: {rust_type}",
            safe_id.name
        )
    } else {
        format!("        {}: {rust_type}", safe_id.name)
    }
}

/// Build derive macro string from default derives plus extra derives
fn build_derive_string(extra_derives: &[String]) -> String {
    let mut all_derives: Vec<String> = DEFAULT_DERIVES.iter().map(|s| s.to_string()).collect();
    all_derives.extend(extra_derives.iter().cloned());
    format!("#[derive({})]", all_derives.join(", "))
}

fn generate_enum(protocol_enum: &ProtocolEnum) -> String {
    let enum_name = &protocol_enum.name;
    let mut out = String::new();

    if let Some(text_str) = &protocol_enum.text {
        out.push_str(&format!("/// {text_str}\n"));
    }

    // For mask enums, generate as a struct with bitflags
    if protocol_enum.is_mask {
        let derives = build_derive_string(&protocol_enum.extra_derives);
        out.push_str(&format!(
            "{}\npub struct {enum_name} {{\n        pub bits: {},\n}}\n\n",
            derives,
            get_rust_type(&protocol_enum.parent)
        ));
    } else {
        // Generate regular enum
        let derives = build_derive_string(&protocol_enum.extra_derives);
        out.push_str(&format!("{}\npub enum ", derives));
        out.push_str(enum_name);
        out.push_str(" {\n");

        for enum_value in &protocol_enum.values {
            let original_name = &enum_value.name;

            // Generate variant name - convert to PascalCase if it has underscores
            let variant_name = if original_name.starts_with("0x") {
                format!("Type{}", &original_name[2..])
            } else {
                original_name.clone()
            };

            // Check if variant name is a reserved word or needs conversion
            let safe_variant = safe_enum_variant_name(&variant_name);

            // Determine if we need serde rename (if the safe name differs from original)
            let needs_serde_rename = safe_variant.name != *original_name;

            if enum_value.value.starts_with("0x") {
                // Hex value
                if needs_serde_rename {
                    out.push_str(&format!(
                        "    #[serde(rename = \"{}\")]\n    {} = {},\n",
                        original_name, safe_variant.name, enum_value.value
                    ));
                } else {
                    out.push_str(&format!(
                        "    {} = {},\n",
                        safe_variant.name, enum_value.value
                    ));
                }
            } else {
                // Decimal value
                if needs_serde_rename {
                    out.push_str(&format!(
                        "    #[serde(rename = \"{}\")]\n    {} = {},\n",
                        original_name, safe_variant.name, enum_value.value
                    ));
                } else {
                    out.push_str(&format!(
                        "    {} = {},\n",
                        safe_variant.name, enum_value.value
                    ));
                }
            }
        }

        out.push_str("}\n\n");
    }

    out
}

/// Analyze all types and add extra derives where needed
fn rectify_dependencies(
    common_types: &[ProtocolType],
    c2s_types: &[ProtocolType], 
    s2c_types: &[ProtocolType],
    enums: &mut Vec<ProtocolEnum>,
    common_types_out: &mut Vec<ProtocolType>,
    c2s_types_out: &mut Vec<ProtocolType>,
    s2c_types_out: &mut Vec<ProtocolType>,
) {
    let mut hash_types = std::collections::HashSet::new();
    
    // Collect all type names that need Hash from all type collections
    for protocol_type in common_types.iter().chain(c2s_types).chain(s2c_types) {
        extract_hash_requirements_from_type(protocol_type, &mut hash_types);
    }
    
    // Add Hash+Eq to enums that need them
    for protocol_enum in enums.iter_mut() {
        if hash_types.contains(&protocol_enum.name) {
            protocol_enum.extra_derives.push("Hash".to_string());
            protocol_enum.extra_derives.push("Eq".to_string());
        }
    }
    
    // Add Hash+Eq to types that need them and copy to output vectors
    for protocol_type in common_types {
        let mut updated_type = protocol_type.clone();
        if hash_types.contains(&protocol_type.name) {
            updated_type.extra_derives.push("Hash".to_string());
            updated_type.extra_derives.push("Eq".to_string());
        }
        common_types_out.push(updated_type);
    }
    
    for protocol_type in c2s_types {
        let mut updated_type = protocol_type.clone();
        if hash_types.contains(&protocol_type.name) {
            updated_type.extra_derives.push("Hash".to_string());
            updated_type.extra_derives.push("Eq".to_string());
        }
        c2s_types_out.push(updated_type);
    }
    
    for protocol_type in s2c_types {
        let mut updated_type = protocol_type.clone();
        if hash_types.contains(&protocol_type.name) {
            updated_type.extra_derives.push("Hash".to_string());
            updated_type.extra_derives.push("Eq".to_string());
        }
        s2c_types_out.push(updated_type);
    }
}

/// Extract type names that need Hash from a single ProtocolType
fn extract_hash_requirements_from_type(
    protocol_type: &ProtocolType,
    hash_types: &mut std::collections::HashSet<String>,
) {
    if let Some(ref field_set) = protocol_type.fields {
        // Check common fields for HashMap usage
        for field in &field_set.common_fields {
            extract_hash_requirements_from_field(&field.field_type, hash_types);
        }

        // Check variant fields for HashMap usage
        if let Some(ref variant_fields) = field_set.variant_fields {
            for case_fields in variant_fields.values() {
                for field in case_fields {
                    extract_hash_requirements_from_field(&field.field_type, hash_types);
                }
            }
        }
    }
}

/// Extract type names that need Hash from a field type string
fn extract_hash_requirements_from_field(
    field_type: &str,
    hash_types: &mut std::collections::HashSet<String>,
) {
    // Look for HashMap<KeyType, V> patterns
    if field_type.starts_with("std::collections::HashMap<") {
        let inner = &field_type["std::collections::HashMap<".len()..field_type.len()-1];
        if let Some(comma_pos) = inner.find(',') {
            let key_type = inner[..comma_pos].trim();
            // Check if the key type is not a single letter generic (like T, U)
            if key_type.len() > 1 && key_type.chars().next().unwrap().is_ascii_uppercase() {
                hash_types.insert(key_type.to_string());
            }
        }
    }
    
    // Also check for PackableHashTable<KeyType, V> patterns
    if field_type.starts_with("PackableHashTable<") {
        let inner = &field_type["PackableHashTable<".len()..field_type.len()-1];
        if let Some(comma_pos) = inner.find(',') {
            let key_type = inner[..comma_pos].trim();
            if key_type.len() > 1 && key_type.chars().next().unwrap().is_ascii_uppercase() {
                hash_types.insert(key_type.to_string());
            }
        }
    }
}

fn generate_type(protocol_type: &ProtocolType) -> String {
    let original_type_name = &protocol_type.name;
    println!("generate_type: name = {original_type_name}");

    // Convert type name to PascalCase
    let safe_type_name = safe_identifier(original_type_name, IdentifierType::Type);
    let type_name = &safe_type_name.name;

    let mut out = String::new();

    if let Some(text_str) = &protocol_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }

    // Handle templated types - we'll generate them as generic structs
    // Continue processing to include fields
    let type_generics = if let Some(template_params) = &protocol_type.templated {
        if protocol_type.hash_bounds.is_empty() {
            format!("<{}>", template_params)
        } else {
            // Add Hash + Eq bounds to generic parameters that need them
            let params: Vec<&str> = template_params.split(',').map(|s| s.trim()).collect();
            let bounded_params: Vec<String> = params.iter().map(|param| {
                if protocol_type.hash_bounds.contains(&param.to_string()) {
                    format!("{}: std::cmp::Eq + std::hash::Hash", param)
                } else {
                    param.to_string()
                }
            }).collect();
            format!("<{}>", bounded_params.join(", "))
        }
    } else {
        String::new()
    };

    // Handle non-templated parent types as type aliases
    // Note: protocol.xml quirk - some types have both parent and templated attributes
    // (e.g., PackableList has parent="List" and templated="T")
    // For templated types, skip the parent alias and continue to struct generation
    if protocol_type.templated.is_none() {
        if let Some(parent_type) = &protocol_type.parent {
            let rust_type = get_rust_type(parent_type);

            // Only generate alias if the rust type differs from the XML type name
            if rust_type != *original_type_name {
                out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
            return out;
        }
    }

    let Some(field_set) = &protocol_type.fields else {
        // No fields, generate empty struct
        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{}}\n\n",
                derives
            ));
        } else {
            out.push_str(&format!(
                "{}\npub struct {type_name}{type_generics} {{}}\n\n",
                derives
            ));
        }
        return out;
    };

    // Check if this is a variant type (has switch) or simple type
    if let Some(ref variant_fields) = field_set.variant_fields {
        // Generate enum
        let switch_field = field_set.switch_field.as_ref().unwrap();

        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{}\n#[serde(rename = \"{original_type_name}\")]
#[serde(tag = \"{switch_field}\")]
pub enum {type_name}{type_generics} {{\n",
                derives
            ));
        } else {
            out.push_str(&format!(
                "{}\n#[serde(tag = \"{switch_field}\")]
pub enum {type_name}{type_generics} {{\n",
                derives
            ));
        }

        // Group case values by their field sets (to handle multi-value cases)
        // Map: field signature -> (primary_value, [all_values])
        let mut field_groups: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();

        for (case_value, case_fields) in variant_fields {
            // Create a signature for these fields to group identical field sets
            let field_sig = case_fields
                .iter()
                .map(|f| format!("{}:{}", f.name, f.field_type))
                .collect::<Vec<_>>()
                .join(";");

            field_groups
                .entry(field_sig)
                .or_insert_with(|| (case_value.clone(), Vec::new()))
                .1
                .push(case_value.clone());
        }

        // Sort by primary value for consistent output
        let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
        sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

        for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
            // Sort values for consistent output
            all_values.sort();

            // Use the first sorted value for both rename and variant name
            let first_value = &all_values[0];

            // Generate variant name from first sorted case value
            let variant_name = if first_value.starts_with("0x") || first_value.starts_with("0X") {
                // Hex value: "0x4" -> "Type4", "0xAB" -> "TypeAB"
                format!("Type{}", &first_value[2..].to_uppercase())
            } else if first_value.starts_with('-') {
                // Negative value: "-1" -> "TypeNeg1"
                format!("TypeNeg{}", &first_value[1..])
            } else {
                // Decimal value: "4" -> "Type4"
                format!("Type{}", first_value)
            };

            // Primary serde rename
            out.push_str(&format!("    #[serde(rename = \"{first_value}\")]\n"));

            // Add aliases for additional values (if multi-value case)
            for alias_value in &all_values[1..] {
                out.push_str(&format!("    #[serde(alias = \"{alias_value}\")]\n"));
            }

            out.push_str(&format!("    {variant_name} {{\n"));

            // Add common fields first (excluding the switch field itself, as serde uses it as the tag)
            for field in &field_set.common_fields {
                if field.name != *switch_field {
                    out.push_str(&generate_field_line(field));
                    out.push_str(",\n");
                }
            }

            // Add variant-specific fields (get from variant_fields using first_value)
            if let Some(case_fields) = variant_fields.get(first_value) {
                for field in case_fields {
                    out.push_str(&generate_field_line(field));
                    out.push_str(",\n");
                }
            }

            out.push_str("    },\n");
        }

        out.push_str("}\n\n");
    } else {
        // Generate struct
        let mut field_out: Vec<String> = Vec::new();

        for field in &field_set.common_fields {
            field_out.push(generate_field_line(field));
        }

        let fields_out: String = field_out.join(",\n");

        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n",
                derives
            ));
        } else {
            out.push_str(&format!(
                "{}\npub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n",
                derives
            ));
        }
    }

    out
}

fn process_switch_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
) -> bool {
    let mut name: Option<String> = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"name" {
            name = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    if let (Some(switch_name), Some(field_set)) = (name, current_field_set) {
        field_set.switch_field = Some(switch_name);
        field_set.variant_fields = Some(BTreeMap::new());
        debug!(
            "Entered switch, switch_field = {:?}",
            field_set.switch_field
        );
        true
    } else {
        false
    }
}

fn process_case_tag(e: &quick_xml::events::BytesStart) -> Option<Vec<String>> {
    let mut value = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"value" {
            value = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    // Parse multi-value cases (e.g., "0x01 | 0x08 | 0x0A") into individual values
    let values = value.map(|v| {
        v.split('|')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
    });

    debug!("current_case_value is now {values:?}");
    values
}

fn process_enum_start_tag(
    e: &quick_xml::events::BytesStart,
    current_enum: &mut Option<ProtocolEnum>,
) {
    let mut name = None;
    let mut text = None;
    let mut parent = None;
    let mut is_mask = false;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"mask" => {
                is_mask = attr.unescape_value().unwrap() == "true";
            }
            _ => {}
        }
    }

    if let Some(name) = name {
        if let Some(parent) = parent {
        let new_enum = ProtocolEnum {
            name,
            text,
            parent,
            values: Vec::new(),
            is_mask,
            extra_derives: Vec::new(),
        };
            *current_enum = Some(new_enum);
        }
    }
}

fn process_enum_value_tag(
    e: &quick_xml::events::BytesStart,
    current_enum: &mut Option<ProtocolEnum>,
) {
    let mut name = None;
    let mut value = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    if let (Some(name), Some(value), Some(current_enum)) = (name, value, current_enum) {
        // Handle multiple values in a single attribute (e.g., "0x0C | 0x0D")
        let values: Vec<&str> = value.split('|').collect();
        for val in values {
            let trimmed_val = val.trim();
            if !trimmed_val.is_empty() {
                let enum_value = EnumValue {
                    name: name.clone(),
                    value: trimmed_val.to_string(),
                };
                current_enum.values.push(enum_value);
            }
        }
    }
}

fn process_field_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    in_switch: bool,
    current_case_values: &Option<Vec<String>>,
    in_if_true: bool,
    in_if_false: bool,
    in_maskmap: bool,
    if_true_fields: &mut Vec<Field>,
    if_false_fields: &mut Vec<Field>,
) {
    let mut field_type = None;
    let mut field_name = None;
    let mut generic_key = None;
    let mut generic_value = None;
    let mut generic_type = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => field_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => field_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericKey" => generic_key = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericValue" => generic_value = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericType" => generic_type = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing field {field_name:?}");

    if let (Some(fname), Some(mut ftype)) = (field_name, field_type) {
        // Handle generic types
        if let (Some(key), Some(value)) = (generic_key, generic_value) {
            // PackableHashTable<K, V>
            ftype = format!("{}<{}, {}>", ftype, key, value);
        } else if let Some(gtype) = generic_type {
            // PackableList<T>
            ftype = format!("{}<{}>", ftype, gtype);
        }
        // Fields in <if> or <maskmap> are optional
        let is_optional = in_if_true || in_if_false || in_maskmap;

        let new_field = Field {
            name: fname,
            field_type: ftype,
            is_optional,
        };

        // If we're in an <if> block, collect fields separately
        if in_if_true {
            if_true_fields.push(new_field);
            debug!("Added field to if_true_fields");
            return;
        } else if in_if_false {
            if_false_fields.push(new_field);
            debug!("Added field to if_false_fields");
            return;
        }

        // Normal field processing
        if let Some(field_set) = current_field_set {
            if in_switch {
                if let (Some(case_vals), Some(variant_fields)) =
                    (current_case_values, &mut field_set.variant_fields)
                {
                    // Add the same field to all values in this case
                    for case_val in case_vals {
                        variant_fields
                            .entry(case_val.clone())
                            .or_insert_with(Vec::new)
                            .push(new_field.clone());
                        debug!("Added field to variant case {case_val}");
                    }
                }
            } else {
                field_set.common_fields.push(new_field);
                debug!("Added field to common_fields");
            }
        }
    }
}

fn process_vector_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    in_switch: bool,
    current_case_values: &Option<Vec<String>>,
    in_if_true: bool,
    in_if_false: bool,
    in_maskmap: bool,
    if_true_fields: &mut Vec<Field>,
    if_false_fields: &mut Vec<Field>,
) {
    let mut vector_type = None;
    let mut vector_name = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => vector_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => vector_name = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing vector {vector_name:?} of type {vector_type:?}");

    if let (Some(vname), Some(vtype)) = (vector_name, vector_type) {
        // Create a field with Vec<T> type
        let vec_type = format!("Vec<{}>", vtype);
        let is_optional = in_if_true || in_if_false || in_maskmap;

        let new_field = Field {
            name: vname,
            field_type: vec_type,
            is_optional,
        };

        // If we're in an <if> block, collect fields separately
        if in_if_true {
            if_true_fields.push(new_field);
            debug!("Added vector to if_true_fields");
            return;
        } else if in_if_false {
            if_false_fields.push(new_field);
            debug!("Added vector to if_false_fields");
            return;
        }

        if let Some(field_set) = current_field_set {
            if in_switch {
                if let (Some(case_vals), Some(variant_fields)) =
                    (current_case_values, &mut field_set.variant_fields)
                {
                    // Add the same field to all values in this case
                    for case_val in case_vals {
                        variant_fields
                            .entry(case_val.clone())
                            .or_insert_with(Vec::new)
                            .push(new_field.clone());
                        debug!("Added vector to variant case {case_val}");
                    }
                }
            } else {
                field_set.common_fields.push(new_field);
                debug!("Added vector to common_fields");
            }
        }
    }
}

fn process_table_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    in_switch: bool,
    current_case_values: &Option<Vec<String>>,
    in_if_true: bool,
    in_if_false: bool,
    in_maskmap: bool,
    if_true_fields: &mut Vec<Field>,
    if_false_fields: &mut Vec<Field>,
) {
    let mut table_name = None;
    let mut key_type = None;
    let mut value_type = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => table_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"key" => key_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value_type = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing table {table_name:?} with key={key_type:?}, value={value_type:?}");

    if let (Some(tname), Some(ktype), Some(vtype)) = (table_name, key_type, value_type) {
        // Create a field with HashMap<K, V> type
        let map_type = format!("std::collections::HashMap<{}, {}>", ktype, vtype);
        let is_optional = in_if_true || in_if_false || in_maskmap;

        let new_field = Field {
            name: tname,
            field_type: map_type,
            is_optional,
        };

        // If we're in an <if> block, collect fields separately
        if in_if_true {
            if_true_fields.push(new_field);
            debug!("Added table to if_true_fields");
            return;
        } else if in_if_false {
            if_false_fields.push(new_field);
            debug!("Added table to if_false_fields");
            return;
        }

        if let Some(field_set) = current_field_set {
            if in_switch {
                if let (Some(case_vals), Some(variant_fields)) =
                    (current_case_values, &mut field_set.variant_fields)
                {
                    // Add the same field to all values in this case
                    for case_val in case_vals {
                        variant_fields
                            .entry(case_val.clone())
                            .or_insert_with(Vec::new)
                            .push(new_field.clone());
                        debug!("Added table to variant case {case_val}");
                    }
                }
            } else {
                field_set.common_fields.push(new_field);
                debug!("Added table to common_fields");
            }
        }
    }
}

fn process_type_tag(
    e: &quick_xml::events::BytesStart,
    is_self_closing: bool,
    types: &mut Vec<ProtocolType>,
    current_type: &mut Option<ProtocolType>,
    current_field_set: &mut Option<FieldSet>,
    filter_types: &Option<Vec<String>>,
) {
    let mut name = None;
    let mut text = None;
    let mut is_primitive = false;
    let mut parent = None;
    let mut templated = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"primitive" => {
                is_primitive = attr.unescape_value().unwrap() == "true";
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"templated" => {
                templated = Some(attr.unescape_value().unwrap().into_owned());
            }
            _ => {}
        }
    }

    if let Some(name) = name {
        let should_skip = filter_types
            .as_ref()
            .map_or(false, |filters| filters.contains(&name));

        if should_skip {
            debug!("Skipping type {name} because it's not in inclusion list.");
            return;
        }

        let new_type = ProtocolType {
            name,
            text,
            fields: None,
            is_primitive,
            rust_type: None,
            parent,
            templated,
            hash_bounds: Vec::new(),
            extra_derives: Vec::new(),
        };

        // For self-closing tags, push immediately
        // For opening tags, set as current_type for further processing
        if is_self_closing {
            types.push(new_type);
        } else {
            *current_type = Some(new_type);
            *current_field_set = Some(FieldSet {
                switch_field: None,
                common_fields: Vec::new(),
                variant_fields: None,
            });
        }

        debug!("Processed type, is_self_closing={is_self_closing}");
    }
}

pub struct GeneratedCode {
    pub enums: String,
    pub common: String,
    pub c2s: String,
    pub s2c: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MessageDirection {
    None,
    C2S,         // <c2s> in messages section
    S2C,         // <s2c> in messages section
    GameActions, // <gameactions> section (C2S)
    GameEvents,  // <gameevents> section (S2C)
}

pub fn generate(xml: &str, filter_types: Option<Vec<String>>) -> GeneratedCode {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    let mut common_types: Vec<ProtocolType> = Vec::new();
    let mut c2s_types: Vec<ProtocolType> = Vec::new();
    let mut s2c_types: Vec<ProtocolType> = Vec::new();
    let mut enums: Vec<ProtocolEnum> = Vec::new();
    let mut current_type: Option<ProtocolType> = None;
    let mut current_enum: Option<ProtocolEnum> = None;
    let mut current_field_set: Option<FieldSet> = None;
    let mut current_case_values: Option<Vec<String>> = None;
    let mut in_switch = false;
    let mut current_direction = MessageDirection::None;

    // Track <if> and <maskmap> state
    let mut in_if = false;
    let mut in_if_true = false;
    let mut in_if_false = false;
    let mut if_true_fields: Vec<Field> = Vec::new();
    let mut if_false_fields: Vec<Field> = Vec::new();
    let mut in_maskmap = false;

    loop {
        let event = reader.read_event_into(&mut buf);

        match event {
            Ok(Event::Start(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "c2s" {
                    current_direction = MessageDirection::C2S;
                    debug!("Entered c2s section");
                } else if tag_name == "s2c" {
                    current_direction = MessageDirection::S2C;
                    debug!("Entered s2c section");
                } else if tag_name == "gameactions" {
                    current_direction = MessageDirection::GameActions;
                    debug!("Entered gameactions section");
                } else if tag_name == "gameevents" {
                    current_direction = MessageDirection::GameEvents;
                    debug!("Entered gameevents section");
                } else if tag_name == "type" {
                    let types_vec = match current_direction {
                        MessageDirection::C2S | MessageDirection::GameActions => &mut c2s_types,
                        MessageDirection::S2C | MessageDirection::GameEvents => &mut s2c_types,
                        MessageDirection::None => &mut common_types,
                    };
                    process_type_tag(
                        &e,
                        false,
                        types_vec,
                        &mut current_type,
                        &mut current_field_set,
                        &filter_types,
                    );
                } else if tag_name == "enum" {
                    process_enum_start_tag(&e, &mut current_enum);
                } else if tag_name == "field" {
                    process_field_tag(
                        &e,
                        &mut current_field_set,
                        in_switch,
                        &current_case_values,
                        in_if_true,
                        in_if_false,
                        in_maskmap,
                        &mut if_true_fields,
                        &mut if_false_fields,
                    );
                } else if tag_name == "switch" {
                    in_switch = process_switch_tag(&e, &mut current_field_set);
                } else if tag_name == "case" {
                    current_case_values = process_case_tag(&e);
                } else if tag_name == "if" {
                    in_if = true;
                    debug!("Entered <if> block");
                } else if tag_name == "true" {
                    in_if_true = true;
                    if_true_fields.clear();
                    debug!("Entered <true> block");
                } else if tag_name == "false" {
                    in_if_false = true;
                    if_false_fields.clear();
                    debug!("Entered <false> block");
                } else if tag_name == "maskmap" {
                    in_maskmap = true;
                    debug!("Entered <maskmap> block");
                } else if tag_name == "mask" {
                    // Masks are treated the same as being in maskmap
                    debug!("Entered <mask> block");
                }
            }
            Ok(Event::Empty(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    let types_vec = match current_direction {
                        MessageDirection::C2S | MessageDirection::GameActions => &mut c2s_types,
                        MessageDirection::S2C | MessageDirection::GameEvents => &mut s2c_types,
                        MessageDirection::None => &mut common_types,
                    };
                    process_type_tag(
                        &e,
                        true,
                        types_vec,
                        &mut current_type,
                        &mut current_field_set,
                        &filter_types,
                    );
                } else if tag_name == "field" {
                    process_field_tag(
                        &e,
                        &mut current_field_set,
                        in_switch,
                        &current_case_values,
                        in_if_true,
                        in_if_false,
                        in_maskmap,
                        &mut if_true_fields,
                        &mut if_false_fields,
                    );
                } else if tag_name == "vector" {
                    process_vector_tag(
                        &e,
                        &mut current_field_set,
                        in_switch,
                        &current_case_values,
                        in_if_true,
                        in_if_false,
                        in_maskmap,
                        &mut if_true_fields,
                        &mut if_false_fields,
                    );
                } else if tag_name == "table" {
                    process_table_tag(
                        &e,
                        &mut current_field_set,
                        in_switch,
                        &current_case_values,
                        in_if_true,
                        in_if_false,
                        in_maskmap,
                        &mut if_true_fields,
                        &mut if_false_fields,
                    );
                } else if tag_name == "value" {
                    process_enum_value_tag(&e, &mut current_enum);
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"type" {
                    // Close out type
                    if let Some(mut ty) = current_type.take() {
                        if !ty.is_primitive {
                            ty.fields = current_field_set.take();
                            // Extract hash bounds for HashMap key types
                            ty.extract_hash_bounds();
                        }
                        let types_vec = match current_direction {
                            MessageDirection::C2S | MessageDirection::GameActions => &mut c2s_types,
                            MessageDirection::S2C | MessageDirection::GameEvents => &mut s2c_types,
                            MessageDirection::None => &mut common_types,
                        };
                        types_vec.push(ty);
                        debug!("DONE with type in {:?} section", current_direction);
                    }
                    in_switch = false;
                    current_case_values = None;
                } else if e.name().as_ref() == b"enum" {
                    // Close out enum
                    if let Some(en) = current_enum.take() {
                        enums.push(en);
                    }
                } else if e.name().as_ref() == b"c2s" {
                    current_direction = MessageDirection::None;
                    debug!("Exited c2s section");
                } else if e.name().as_ref() == b"s2c" {
                    current_direction = MessageDirection::None;
                    debug!("Exited s2c section");
                } else if e.name().as_ref() == b"gameactions" {
                    current_direction = MessageDirection::None;
                    debug!("Exited gameactions section");
                } else if e.name().as_ref() == b"gameevents" {
                    current_direction = MessageDirection::None;
                    debug!("Exited gameevents section");
                } else if e.name().as_ref() == b"switch" {
                    in_switch = false;
                    debug!("Exited switch");
                } else if e.name().as_ref() == b"case" {
                    current_case_values = None;
                } else if e.name().as_ref() == b"if" {
                    // Merge if_true and if_false fields and add to current_field_set
                    let merged_fields = merge_if_fields(
                        std::mem::take(&mut if_true_fields),
                        std::mem::take(&mut if_false_fields),
                    );

                    if let Some(field_set) = &mut current_field_set {
                        for field in merged_fields {
                            if in_switch {
                                if let (Some(case_vals), Some(variant_fields)) =
                                    (&current_case_values, &mut field_set.variant_fields)
                                {
                                    // Add field to all current case values
                                    for case_val in case_vals {
                                        variant_fields
                                            .entry(case_val.clone())
                                            .or_insert_with(Vec::new)
                                            .push(field.clone());
                                    }
                                }
                            } else {
                                field_set.common_fields.push(field);
                            }
                        }
                    }

                    in_if = false;
                    debug!("Exited <if> block");
                } else if e.name().as_ref() == b"true" {
                    in_if_true = false;
                    debug!("Exited <true> block");
                } else if e.name().as_ref() == b"false" {
                    in_if_false = false;
                    debug!("Exited <false> block");
                } else if e.name().as_ref() == b"maskmap" {
                    in_maskmap = false;
                    debug!("Exited <maskmap> block");
                } else if e.name().as_ref() == b"mask" {
                    debug!("Exited <mask> block");
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    // Rectify dependencies between types and enums
    let mut rectified_common_types = Vec::new();
    let mut rectified_c2s_types = Vec::new();
    let mut rectified_s2c_types = Vec::new();
    rectify_dependencies(
        &common_types, 
        &c2s_types, 
        &s2c_types, 
        &mut enums,
        &mut rectified_common_types,
        &mut rectified_c2s_types,
        &mut rectified_s2c_types,
    );

    // Helper function to generate code for a list of types
    let generate_types_code = |types: &Vec<ProtocolType>| -> String {
        let mut out = String::new();
        out.push_str("use serde::{Serialize, Deserialize};\n\n");

        for protocol_type in types {
            if protocol_type.is_primitive {
                // Generate type alias for primitive types
                let type_name = &protocol_type.name;
                let rust_type = get_rust_type(type_name);

                // Only generate alias if the rust type differs from the XML type name
                if rust_type != type_name {
                    if let Some(ref text) = protocol_type.text {
                        out.push_str(&format!("/// {text}\n"));
                    }
                    out.push_str(&format!(
                        "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                    ));
                }
            } else {
                out.push_str(&generate_type(protocol_type));
            }
        }
        out
    };

    // Generate enums
    let mut enums_out = String::new();
    enums_out.push_str("use serde::{Serialize, Deserialize};\n\n");

    for protocol_enum in &enums {
        enums_out.push_str(&generate_enum(protocol_enum));
    }

    // Generate common types
    let mut common_out = String::new();
    common_out.push_str("use serde::{Serialize, Deserialize};\n\n");

    for protocol_type in &rectified_common_types {
        if protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let rust_type = get_rust_type(type_name);
            if rust_type != type_name {
                if let Some(ref text) = protocol_type.text {
                    common_out.push_str(&format!("/// {text}\n"));
                }
                common_out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
        } else {
            common_out.push_str(&generate_type(protocol_type));
        }
    }

    GeneratedCode {
        enums: enums_out,
        common: common_out,
        c2s: generate_types_code(&rectified_c2s_types),
        s2c: generate_types_code(&rectified_s2c_types),
    }
}
