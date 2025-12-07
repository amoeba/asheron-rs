use core::panic;
use std::collections::BTreeMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    identifiers::{IdentifierType, safe_enum_variant_name, safe_identifier},
    types::{EnumValue, Field, FieldSet, IfBranch, ProtocolEnum, ProtocolType, Subfield},
    util::{format_hex_value, parse_hex_string},
};

mod identifiers;
mod types;
mod util;

/// Context for field processing within conditional blocks
#[derive(Debug)]
struct FieldContext {
    in_switch: bool,
    current_case_values: Option<Vec<i64>>, // Parsed numeric case values
    in_if_true: bool,
    in_if_false: bool,
    in_maskmap: bool,
    if_true_fields: Vec<Field>,
    if_false_fields: Vec<Field>,
    /// The test condition from the current <if> block (e.g., "RecordCount > 0")
    current_if_condition: Option<String>,
    /// The field name being used for maskmap (e.g., "Flags")
    current_maskmap_field: Option<String>,
    /// The current mask value (e.g., "0x8")
    current_mask_value: Option<String>,
    /// Track when we're inside a field tag (to collect subfields)
    in_field: bool,
    /// The current field being built (accumulates subfields)
    current_field: Option<Field>,
}

/// Context for code generation, controlling what gets generated
#[derive(Debug, Clone)]
pub struct GenerationContext {
    /// List of type names to generate readers for
    /// Empty list means no readers
    /// ["*"] means all readers
    filter_types: Vec<String>,
}

impl GenerationContext {
    pub fn new(filter_types: Vec<String>) -> Self {
        Self { filter_types }
    }

    /// Check if readers should be generated for the given type name
    pub fn should_generate_reader(&self, type_name: &str) -> bool {
        if self.filter_types.is_empty() {
            return false;
        }

        if self.filter_types.contains(&"*".to_string()) {
            return true;
        }

        self.filter_types.contains(&type_name.to_string())
    }
}

/// Context for reader generation containing type information
pub struct ReaderContext {
    /// Map from enum name to its parent type (e.g., "NetAuthType" -> "uint")
    enum_parent_map: std::collections::HashMap<String, String>,
    /// Map from (enum_name, value) to variant name (e.g., ("NetAuthType", 2) -> "AccountPassword")
    enum_value_map: std::collections::HashMap<(String, i64), String>,
}

impl ReaderContext {
    pub fn new(
        enum_parent_map: std::collections::HashMap<String, String>,
        enum_value_map: std::collections::HashMap<(String, i64), String>,
    ) -> Self {
        Self {
            enum_parent_map,
            enum_value_map,
        }
    }
}

/// Check if a Rust type is a primitive (including String)
fn is_primitive_type(rust_type: &str) -> bool {
    matches!(
        rust_type,
        "u8" | "i8"
            | "u16"
            | "i16"
            | "u32"
            | "i32"
            | "u64"
            | "i64"
            | "f32"
            | "f64"
            | "bool"
            | "String"
    )
}

/// Check if a type should be generated as a newtype struct (vs type alias)
/// Returns true if it's a semantic type with a primitive parent (not a C-style alias)
fn should_be_newtype_struct(type_name: &str, rust_parent_type: &str) -> bool {
    // WString is always a newtype struct (has custom UTF-16 wire format)
    if type_name == "WString" {
        return true;
    }

    // Only consider newtype for types with primitive parents
    if !is_primitive_type(rust_parent_type) {
        return false;
    }

    // Hybrid approach: Use newtype structs for semantic types (type safety)
    // but keep type aliases for C-style primitive names (WORD, DWORD, byte, etc.)
    // Heuristic: C-style aliases are all-uppercase or all-lowercase
    let is_all_upper = type_name
        .chars()
        .all(|c| c.is_uppercase() || !c.is_alphabetic());
    let is_all_lower = type_name
        .chars()
        .all(|c| c.is_lowercase() || !c.is_alphabetic());
    let is_c_style_alias = is_all_upper || is_all_lower;

    !is_c_style_alias
}

/// Default derives for all generated types
const DEFAULT_DERIVES: &[&str] = &["Clone", "Debug", "PartialEq", "Serialize", "Deserialize"];

/// Default derives for all generated enums (includes TryFromPrimitive for safe conversion)
const DEFAULT_ENUM_DERIVES: &[&str] = &[
    "Clone",
    "Debug",
    "PartialEq",
    "Serialize",
    "Deserialize",
    "TryFromPrimitive",
];

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
        "string" => "String",
        // WString is kept as-is so it becomes a newtype struct
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

/// Merge fields from <if test="..."> <true> and <false> branches
///
/// This preserves branch information for code generation. The generated code will emit
/// if-else blocks that read different fields in each branch, rather than making all
/// fields optional.
///
/// Logic:
/// - Fields only in one branch: keep with IfBranch set (True or False)
/// - Fields in both branches with same type: mark as IfBranch::Both, make optional
/// - Fields in both branches with different types: use larger type, mark as IfBranch::Both,
///   make optional, and store the false branch type for casting during code generation
fn merge_if_fields(mut true_fields: Vec<Field>, mut false_fields: Vec<Field>) -> Vec<Field> {
    use std::collections::HashMap;

    let mut true_map: HashMap<String, Field> =
        true_fields.drain(..).map(|f| (f.name.clone(), f)).collect();
    let mut false_map: HashMap<String, Field> = false_fields
        .drain(..)
        .map(|f| (f.name.clone(), f))
        .collect();

    let mut result = Vec::new();

    // Process all unique field names
    let all_names: std::collections::HashSet<_> =
        true_map.keys().chain(false_map.keys()).cloned().collect();

    for name in all_names {
        match (true_map.remove(&name), false_map.remove(&name)) {
            (Some(mut true_field), Some(false_field)) => {
                // Field exists in both branches
                if true_field.field_type != false_field.field_type {
                    // Different types - use the larger one and store the false branch type
                    let larger_type =
                        get_larger_type(&true_field.field_type, &false_field.field_type);
                    true_field.if_false_branch_type = Some(false_field.field_type.clone());
                    true_field.field_type = larger_type;
                }
                // Mark as existing in both branches and make optional
                true_field.if_branch = Some(IfBranch::Both);
                true_field.is_optional = true;
                result.push(true_field);
            }
            (Some(field), None) => {
                // Field only in true branch - keep IfBranch::True
                result.push(field);
            }
            (None, Some(field)) => {
                // Field only in false branch - keep IfBranch::False
                result.push(field);
            }
            (None, None) => unreachable!(),
        }
    }

    // Sort by name for consistent output
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

fn generate_field_line(field: &Field, is_enum_variant: bool) -> String {
    let original_name = &field.name;
    let safe_id = safe_identifier(original_name, IdentifierType::Field);
    let mut rust_type = get_rust_type(&field.field_type).to_string();

    // Wrap in Option if the field is optional
    if field.is_optional {
        rust_type = format!("Option<{rust_type}>");
    }

    // Enum variant fields can't have pub visibility
    let visibility = if is_enum_variant { "" } else { "pub " };

    if safe_id.needs_rename {
        format!(
            "    #[serde(rename = \"{original_name}\")]\n    {}{}: {rust_type}",
            visibility, safe_id.name
        )
    } else {
        format!("    {}{}: {rust_type}", visibility, safe_id.name)
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
        let parent_rust = get_rust_type(&protocol_enum.parent);
        let read_fn = match parent_rust {
            "u8" => "read_u8",
            "i8" => "read_i8",
            "u16" => "read_u16",
            "i16" => "read_i16",
            "u32" => "read_u32",
            "i32" => "read_i32",
            "u64" => "read_u64",
            "i64" => "read_i64",
            _ => "",
        };
        if !read_fn.is_empty() {
            out.push_str(&format!(
                "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        let value = crate::readers::{read_fn}(reader)?;\n        Ok({}::try_from(value)?)\n    }}\n}}\n\n",
                enum_name, enum_name
            ));
        }
    }

    out
}

/// Analyze all types and add extra derives where needed
fn rectify_dependencies(
    common_types: &[ProtocolType],
    c2s_types: &[ProtocolType],
    s2c_types: &[ProtocolType],
    enums: &mut [ProtocolEnum],
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
        let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
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
        let inner = &field_type["PackableHashTable<".len()..field_type.len() - 1];
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
            format!("<{template_params}>")
        } else {
            // Add Hash + Eq bounds to generic parameters that need them
            let params: Vec<&str> = template_params.split(',').map(|s| s.trim()).collect();
            let bounded_params: Vec<String> = params
                .iter()
                .map(|param| {
                    if protocol_type.hash_bounds.contains(&param.to_string()) {
                        format!("{param}: std::cmp::Eq + std::hash::Hash")
                    } else {
                        param.to_string()
                    }
                })
                .collect();
            format!("<{}>", bounded_params.join(", "))
        }
    } else {
        String::new()
    };

    // Handle non-templated parent types as type aliases or newtype structs
    // Note: protocol.xml quirk - some types have both parent and templated attributes
    // (e.g., PackableList has parent="List" and templated="T")
    // For templated types, skip the parent alias and continue to struct generation
    if protocol_type.templated.is_none()
        && let Some(parent_type) = &protocol_type.parent
    {
        let rust_type = get_rust_type(parent_type);

        // Only generate if the rust type differs from the XML type name
        if rust_type != *original_type_name {
            if should_be_newtype_struct(type_name, rust_type) {
                // Generate newtype struct for semantic types
                // WString wraps String (not Copy), others wrap primitives (Copy)
                if type_name == "WString" {
                    out.push_str("#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]\n");
                } else {
                    out.push_str(
                        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]\n",
                    );
                }
                out.push_str("#[derive(serde::Serialize, serde::Deserialize)]\n");
                out.push_str("#[serde(transparent)]\n");
                out.push_str(&format!("pub struct {type_name}(pub {rust_type});\n\n"));
            } else {
                // Generate type alias for C-style aliases or non-primitive parents
                out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
        }
        return out;
    }

    let Some(field_set) = &protocol_type.fields else {
        // No fields, generate empty struct
        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{derives}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{}}\n\n"
            ));
        } else {
            out.push_str(&format!(
                "{derives}\npub struct {type_name}{type_generics} {{}}\n\n"
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
                "{derives}\n#[serde(rename = \"{original_type_name}\")]
#[serde(tag = \"{switch_field}\")]
pub enum {type_name}{type_generics} {{\n"
            ));
        } else {
            out.push_str(&format!(
                "{derives}\n#[serde(tag = \"{switch_field}\")]
pub enum {type_name}{type_generics} {{\n"
            ));
        }

        // Group case values by their field sets (to handle multi-value cases)
        // Map: field signature -> (primary_value, [all_values])
        let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

        for (case_value, case_fields) in variant_fields {
            // Create a signature for these fields to group identical field sets
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

        for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
            // Sort values for consistent output
            all_values.sort();

            // Use the first sorted value for both rename and variant name
            let first_value = all_values[0];
            let first_value_str = format_hex_value(first_value);

            // Generate variant name from first sorted case value
            let variant_name = if first_value < 0 {
                // Negative value: -1 -> "TypeNeg1"
                format!("TypeNeg{}", first_value.abs())
            } else {
                // Positive value: format as hex and extract hex digits
                // 0x04 -> "04", 0xAB -> "AB"
                let hex_str = format!("{:X}", first_value);
                format!("Type{}", hex_str)
            };

            // Primary serde rename
            out.push_str(&format!("    #[serde(rename = \"{first_value_str}\")]\n"));

            // Add aliases for additional values (if multi-value case)
            for alias_value in &all_values[1..] {
                let alias_str = format_hex_value(*alias_value);
                out.push_str(&format!("    #[serde(alias = \"{alias_str}\")]\n"));
            }

            out.push_str(&format!("    {variant_name} {{\n"));

            // Add common fields first (excluding the switch field itself, as serde uses it as the tag)
            for field in &field_set.common_fields {
                if field.name != *switch_field {
                    out.push_str(&generate_field_line(field, true)); // true = is enum variant
                    out.push_str(",\n");
                }
            }

            // Add variant-specific fields (get from variant_fields using first_value)
            if let Some(case_fields) = variant_fields.get(&first_value) {
                for field in case_fields {
                    out.push_str(&generate_field_line(field, true)); // true = is enum variant
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
            field_out.push(generate_field_line(field, false)); // false = is struct field
        }

        let fields_out: String = field_out.join(",\n") + ",";

        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{derives}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n"
            ));
        } else {
            out.push_str(&format!(
                "{derives}\npub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n"
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

fn process_case_tag(e: &quick_xml::events::BytesStart) -> Option<Vec<i64>> {
    let mut value = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"value" {
            value = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    // Parse multi-value cases (e.g., "0x01 | 0x08 | 0x0A") into individual parsed i64 values
    let values = value.map(|v| {
        v.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                parse_hex_string(s)
                    .map_err(|e| {
                        eprintln!("Warning: {}", e);
                        e
                    })
                    .ok()
            })
            .collect::<Vec<i64>>()
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

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            _ => {}
        }
    }

    if let Some(name) = name
        && let Some(parent) = parent
    {
        let new_enum = ProtocolEnum {
            name,
            text,
            parent,
            values: Vec::new(),
            extra_derives: Vec::new(),
        };
        *current_enum = Some(new_enum);
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

    if let (Some(name), Some(value_str), Some(current_enum)) = (name, value, current_enum) {
        // Handle multiple values in a single attribute (e.g., "0x0C | 0x0D")
        let values: Vec<&str> = value_str.split('|').collect();
        for val in values {
            let trimmed_val = val.trim();
            if !trimmed_val.is_empty() {
                match parse_hex_string(trimmed_val) {
                    Ok(parsed_value) => {
                        let enum_value = EnumValue {
                            name: name.clone(),
                            value: parsed_value,
                        };
                        current_enum.values.push(enum_value);
                    }
                    Err(e) => {
                        eprintln!("Warning: {}", e);
                    }
                }
            }
        }
    }
}

fn process_subfield_tag(e: &quick_xml::events::BytesStart) -> Option<Subfield> {
    let mut name = None;
    let mut field_type = None;
    let mut value_expression = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"type" => field_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value_expression = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    if let (Some(name), Some(field_type), Some(value_expression)) = (name, field_type, value_expression) {
        Some(Subfield {
            name,
            field_type,
            value_expression,
        })
    } else {
        None
    }
}

fn create_field_from_tag(
    e: &quick_xml::events::BytesStart,
    ctx: &FieldContext,
) -> Option<Field> {
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
            ftype = format!("{ftype}<{key}, {value}>");
        } else if let Some(gtype) = generic_type {
            // PackableList<T>
            ftype = format!("{ftype}<{gtype}>");
        }
        // Fields inside <if>/<true>/<false> or <maskmap> blocks are conditional
        let is_optional = ctx.in_if_true || ctx.in_if_false || ctx.in_maskmap;

        // Track which branch this field belongs to (used for if/true/false code generation)
        let if_branch = if ctx.in_if_true {
            Some(IfBranch::True)
        } else if ctx.in_if_false {
            Some(IfBranch::False)
        } else {
            None
        };

        Some(Field {
            name: fname,
            field_type: ftype,
            is_optional,
            length_expression: None, // Regular fields don't have length expressions
            optional_condition: if is_optional {
                ctx.current_if_condition.clone()
            } else {
                None
            },
            mask_field: if ctx.in_maskmap {
                ctx.current_maskmap_field.clone()
            } else {
                None
            },
            mask_value: if ctx.in_maskmap {
                ctx.current_mask_value.clone()
            } else {
                None
            },
            if_branch,
            if_false_branch_type: None,
            subfields: Vec::new(),
        })
    } else {
        None
    }
}

fn add_field_to_set(
    field: Field,
    current_field_set: &mut Option<FieldSet>,
    ctx: &FieldContext,
) {
    // If we're in an <if> block, this shouldn't be called - fields are handled separately
    // This is just for normal fields

    // Normal field processing
    if let Some(field_set) = current_field_set {
        if ctx.in_switch {
            if let (Some(case_vals), Some(variant_fields)) =
                (&ctx.current_case_values, &mut field_set.variant_fields)
            {
                // Add the same field to all values in this case
                for case_val in case_vals {
                    variant_fields
                        .entry(case_val.clone())
                        .or_insert_with(Vec::new)
                        .push(field.clone());
                    debug!("Added field to variant case {case_val}");
                }
            }
        } else {
            field_set.common_fields.push(field);
            debug!("Added field to common_fields");
        }
    }
}

fn process_field_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    if let Some(new_field) = create_field_from_tag(e, ctx) {
        // If we're in an <if> block, collect fields separately
        if ctx.in_if_true {
            ctx.if_true_fields.push(new_field);
            debug!("Added field to if_true_fields");
            return;
        } else if ctx.in_if_false {
            ctx.if_false_fields.push(new_field);
            debug!("Added field to if_false_fields");
            return;
        }

        add_field_to_set(new_field, current_field_set, ctx);
    }
}

fn process_vector_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    let mut vector_type = None;
    let mut vector_name = None;
    let mut length_expr = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => vector_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => vector_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"length" => length_expr = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing vector {vector_name:?} of type {vector_type:?} with length {length_expr:?}");

    if let (Some(vname), Some(vtype)) = (vector_name, vector_type) {
        // Create a field with Vec<T> type
        let vec_type = format!("Vec<{vtype}>");
        let is_optional = ctx.in_if_true || ctx.in_if_false || ctx.in_maskmap;

        let if_branch = if ctx.in_if_true {
            Some(IfBranch::True)
        } else if ctx.in_if_false {
            Some(IfBranch::False)
        } else {
            None
        };

        let new_field = Field {
            name: vname,
            field_type: vec_type,
            is_optional,
            length_expression: length_expr,
            optional_condition: if is_optional {
                ctx.current_if_condition.clone()
            } else {
                None
            },
            mask_field: if ctx.in_maskmap {
                ctx.current_maskmap_field.clone()
            } else {
                None
            },
            mask_value: if ctx.in_maskmap {
                ctx.current_mask_value.clone()
            } else {
                None
            },
            if_branch,
            if_false_branch_type: None,
            subfields: Vec::new(),
        };

        // If we're in an <if> block, collect fields separately
        if ctx.in_if_true {
            ctx.if_true_fields.push(new_field);
            debug!("Added vector to if_true_fields");
            return;
        } else if ctx.in_if_false {
            ctx.if_false_fields.push(new_field);
            debug!("Added vector to if_false_fields");
            return;
        }

        if let Some(field_set) = current_field_set {
            if ctx.in_switch {
                if let (Some(case_vals), Some(variant_fields)) =
                    (&ctx.current_case_values, &mut field_set.variant_fields)
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
            }
        }
    }
}

fn process_table_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    let mut table_name = None;
    let mut key_type = None;
    let mut value_type = None;
    let mut length_expr = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => table_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"key" => key_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"length" => length_expr = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!(
        "Processing table {table_name:?} with key={key_type:?}, value={value_type:?}, length={length_expr:?}"
    );

    if let (Some(tname), Some(ktype), Some(vtype)) = (table_name, key_type, value_type) {
        // Create a field with HashMap<K, V> type
        let map_type = format!("std::collections::HashMap<{ktype}, {vtype}>");
        let is_optional = ctx.in_if_true || ctx.in_if_false || ctx.in_maskmap;

        let if_branch = if ctx.in_if_true {
            Some(IfBranch::True)
        } else if ctx.in_if_false {
            Some(IfBranch::False)
        } else {
            None
        };

        let new_field = Field {
            name: tname,
            field_type: map_type,
            is_optional,
            length_expression: length_expr,
            optional_condition: if is_optional {
                ctx.current_if_condition.clone()
            } else {
                None
            },
            mask_field: if ctx.in_maskmap {
                ctx.current_maskmap_field.clone()
            } else {
                None
            },
            mask_value: if ctx.in_maskmap {
                ctx.current_mask_value.clone()
            } else {
                None
            },
            if_branch,
            if_false_branch_type: None,
            subfields: Vec::new(),
        };

        // If we're in an <if> block, collect fields separately
        if ctx.in_if_true {
            ctx.if_true_fields.push(new_field);
            debug!("Added table to if_true_fields");
            return;
        } else if ctx.in_if_false {
            ctx.if_false_fields.push(new_field);
            debug!("Added table to if_false_fields");
            return;
        }

        if let Some(field_set) = current_field_set {
            if ctx.in_switch {
                if let (Some(case_vals), Some(variant_fields)) =
                    (&ctx.current_case_values, &mut field_set.variant_fields)
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
        let new_type = ProtocolType {
            name,
            text,
            fields: None,
            is_primitive,
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
    pub readers_common: String,
    pub readers_c2s: String,
    pub readers_s2c: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MessageDirection {
    None,
    C2S,         // <c2s> in messages section
    S2C,         // <s2c> in messages section
    GameActions, // <gameactions> section (C2S)
    GameEvents,  // <gameevents> section (S2C)
}

/// Generate readers for a list of types
fn generate_readers_for_types(
    ctx: &GenerationContext,
    types: &[ProtocolType],
    enums: &[ProtocolEnum],
    module_name: &str,
) -> String {
    let mut out = String::new();
    out.push_str("// Binary readers for ");
    out.push_str(module_name);
    out.push_str(" types\n\n");
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use std::io::Read;\n");
    out.push_str("#[allow(unused_imports)]\n");

    // Common types are directly in crate::types, c2s/s2c are in submodules
    if module_name == "common" {
        out.push_str("use crate::types::*;\n");
    } else {
        out.push_str("use crate::types::*;\n");
        out.push_str("use crate::types::");
        out.push_str(module_name);
        out.push_str("::*;\n");
    }

    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::enums::*;\n");
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use super::*;\n\n");

    // Build a map of enum names to their parent types for quick lookup
    let enum_parent_map: std::collections::HashMap<String, String> = enums
        .iter()
        .map(|e| (e.name.clone(), e.parent.clone()))
        .collect();

    // Build a map of (enum_name, value) -> variant_name for switch pattern matching
    let mut enum_value_map: std::collections::HashMap<(String, i64), String> =
        std::collections::HashMap::new();
    for protocol_enum in enums {
        for enum_value in &protocol_enum.values {
            // Use safe enum variant name when referring to variant
            // i.e., EmoteType::RefuseEmoteCategory instead of EmoteType::Refuse_EmoteCategory
            let safe_variant = safe_enum_variant_name(&enum_value.name);

            // Values are already parsed as i64, no normalization needed
            enum_value_map.insert(
                (protocol_enum.name.clone(), enum_value.value),
                safe_variant.name,
            );
        }
    }

    // Create reader context with enum information
    let reader_ctx = ReaderContext::new(enum_parent_map, enum_value_map);

    for protocol_type in types {
        if ctx.should_generate_reader(&protocol_type.name) {
            out.push_str(&generate_reader_impl(&reader_ctx, protocol_type));
        }
    }

    out
}

/// Generate a reader implementation for a single type (as an impl block on the type)
fn generate_reader_impl(ctx: &ReaderContext, protocol_type: &ProtocolType) -> String {
    let type_name = &protocol_type.name;
    let safe_type_name = safe_identifier(type_name, IdentifierType::Type);

    // Primitive types (u32, bool, etc.) have read_* helper functions
    if protocol_type.is_primitive {
        return String::new();
    }

    // Templated/generic types (PackableList<T>, PackableHashTable<T,U>, etc.)
    // use generic helper functions like read_packable_list() instead of impl blocks
    if protocol_type.templated.is_some() {
        return String::new();
    }

    // Handle newtype structs (parent type with no fields, but not C-style aliases)
    // These are generated as newtype structs and need a read() method
    if protocol_type.parent.is_some() && protocol_type.fields.is_none() {
        if let Some(parent_type) = &protocol_type.parent {
            let rust_type = get_rust_type(parent_type);

            if should_be_newtype_struct(&safe_type_name.name, rust_type) {
                // Generate read() for newtype struct
                // Strategy: Try to use a read_* helper function
                // - For numeric primitives: use read_u32(), read_i16(), etc.
                // - For String/WString types: use read_{typename}() helper
                let read_call = match rust_type {
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
                    // String or custom parent: call read_{lowercase_typename}()
                    _ => format!("read_{}(reader)", safe_type_name.name.to_lowercase()),
                };

                let impl_code = format!(
                    "impl {} {{\n    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        Ok(Self({}?))\n    }}\n}}\n\n",
                    safe_type_name.name, read_call
                );
                let acdatatype_code = format!(
                    "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
                    safe_type_name.name, safe_type_name.name
                );
                return format!("{}{}", impl_code, acdatatype_code);
            }
        }
        // Type alias - doesn't need a reader
        return String::new();
    }

    let Some(field_set) = &protocol_type.fields else {
        // Empty struct - no fields to read
        let impl_code = format!(
            "impl {} {{\n    pub fn read(_reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        Ok(Self {{}})\n    }}\n}}\n\n",
            safe_type_name.name
        );
        let acdatatype_code = format!(
            "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
            safe_type_name.name, safe_type_name.name
        );
        return format!("{}{}", impl_code, acdatatype_code);
    };

    // Check if this is a variant type (has switch)
    if field_set.variant_fields.is_some() {
        generate_variant_reader_impl(ctx, protocol_type, &safe_type_name.name, field_set)
    } else {
        generate_struct_reader_impl(ctx, protocol_type, &safe_type_name.name, field_set)
    }
}

/// Generate a reader for a simple struct (no variants)
fn generate_struct_reader_impl(
    ctx: &ReaderContext,
    _protocol_type: &ProtocolType,
    type_name: &str,
    field_set: &FieldSet,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {} {{\n", type_name));
    out.push_str(
        "    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {\n",
    );

    // Group consecutive fields with the same condition
    let field_groups = group_consecutive_fields_by_condition(&field_set.common_fields);

    // Generate reads for each group
    for group in &field_groups {
        generate_field_group_reads(
            ctx,
            &mut out,
            &group.condition,
            &group.fields,
            &field_set.common_fields,
        );
    }

    // Construct the struct
    out.push_str("\n        Ok(Self {\n");
    for field in &field_set.common_fields {
        let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
        out.push_str(&format!("            {},\n", field_name));
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACDataType implementation
    out.push_str(&format!(
        "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
        type_name, type_name
    ));

    out
}

/// A group of consecutive fields with the same condition
#[derive(Debug)]
struct FieldGroup<'a> {
    condition: ConditionKey,
    fields: Vec<&'a Field>,
}

/// Group consecutive fields with the same condition for coalescing conditional reads
/// This preserves field order from the XML
fn group_consecutive_fields_by_condition(fields: &[Field]) -> Vec<FieldGroup<'_>> {
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

/// Key for grouping fields by their condition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ConditionKey {
    /// No condition - always read
    None,
    /// <if test="condition">
    IfTest(String),
    /// <maskmap> with specific field and mask value
    Mask { field: String, value: String },
}

impl ConditionKey {
    fn from_field(field: &Field) -> Self {
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

/// Generate code to assign a field that appears in both branches
///
/// Since the variable is pre-declared with `let field;`, we just assign to it.
/// This avoids the unused_assignments warning and is cleaner.
fn generate_both_branch_field_read_no_let(
    ctx: &ReaderContext,
    field: &Field,
    all_fields: &[Field],
    field_name: &str,
    out: &mut String,
) {
    // Check if this field has a different type in the false branch
    if let Some(false_type_str) = &field.if_false_branch_type {
        let rust_type_merged = get_rust_type(&field.field_type);
        let rust_type_false = get_rust_type(false_type_str);

        if rust_type_false != rust_type_merged {
            // Types differ - read the smaller type from false branch and cast to merged type
            let mut false_field_temp = (*field).clone();
            false_field_temp.field_type = false_type_str.clone();
            let false_read_call = generate_base_read_call(ctx, &false_field_temp, all_fields);
            out.push_str(&format!(
                "            {} = Some(({})? as {});\n",
                field_name, false_read_call, rust_type_merged
            ));
        } else {
            // Types match - just read normally
            out.push_str(&format!(
                "            {} = Some({}?);\n",
                field_name,
                generate_base_read_call(ctx, field, all_fields)
            ));
        }
    } else {
        // No type difference recorded, read the merged type normally
        out.push_str(&format!(
            "            {} = Some({}?);\n",
            field_name,
            generate_base_read_call(ctx, field, all_fields)
        ));
    }
}

/// Generate reads for a group of fields with the same condition
fn generate_field_group_reads(
    ctx: &ReaderContext,
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
                let read_call = generate_base_read_call(ctx, field, all_fields);
                out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));

                // Generate subfield computations if any
                for subfield in &field.subfields {
                    let subfield_name = safe_identifier(&subfield.name, IdentifierType::Field).name;
                    let subfield_expr = convert_condition_expression(&subfield.value_expression, all_fields);
                    let subfield_rust_type = get_rust_type(&subfield.field_type);
                    out.push_str(&format!("        let {} = ({}) as {};\n", subfield_name, subfield_expr, subfield_rust_type));
                }
            }
        }
        ConditionKey::IfTest(condition) => {
            // Handle <if test="condition"> with <true> and <false> branches
            let rust_condition = convert_condition_expression(condition, all_fields);

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
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name,
                    generate_base_read_call(ctx, field, all_fields)
                ));
            }
            for field in &both {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name,
                    generate_base_read_call(ctx, field, all_fields)
                ));
            }

            // FALSE branch: only emit if there are fields to read in the false branch
            if !false_only.is_empty() || !both.is_empty() {
                out.push_str("        } else {\n");

                // Read false-only fields
                for field in &false_only {
                    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    out.push_str(&format!(
                        "            {} = Some({}?);\n",
                        field_name,
                        generate_base_read_call(ctx, field, all_fields)
                    ));
                }

                // Read both-branch fields, with type casting if needed
                for field in &both {
                    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    generate_both_branch_field_read_no_let(
                        ctx,
                        field,
                        all_fields,
                        &field_name,
                        &mut *out,
                    );
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
            // Generate a single if block for all fields with this mask
            let mask_field_safe = safe_identifier(mask_field, IdentifierType::Field).name;
            let mask_value_code = if mask_value.contains('.') {
                let parts: Vec<&str> = mask_value.split('.').collect();
                if parts.len() == 2 {
                    format!("{}::{} as u32", parts[0], parts[1])
                } else {
                    mask_value.clone()
                }
            } else {
                mask_value.clone()
            };

            // Initialize all optional fields to None first
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let mut {} = None;\n", field_name));
            }

            // Generate the if block
            out.push_str(&format!(
                "        if ({} & {}) != 0 {{\n",
                mask_field_safe, mask_value_code
            ));
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                let read_call = generate_base_read_call(ctx, field, all_fields);
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name, read_call
                ));
            }
            out.push_str("        }\n");
        }
    }
}

/// Generate the base read call without the conditional wrapper
fn generate_base_read_call(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let field_type = &field.field_type;
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
        _ => {
            // Check if it's an enum
            if let Some(parent_type) = ctx.enum_parent_map.get(field_type) {
                let parent_rust_type = get_rust_type(parent_type);
                let read_fn = match parent_rust_type {
                    "u8" => "read_u8",
                    "i8" => "read_i8",
                    "u16" => "read_u16",
                    "i16" => "read_i16",
                    "u32" => "read_u32",
                    "i32" => "read_i32",
                    "u64" => "read_u64",
                    "i64" => "read_i64",
                    _ => panic!("Unsupported enum parent type: {}", parent_type),
                };
                format!("{}::try_from({}(reader)?)", field_type, read_fn)
            } else if field_type.starts_with("Vec<") {
                let element_type = &field_type[4..field_type.len() - 1];
                if let Some(length_expr) = &field.length_expression {
                    let length_code = convert_length_expression(length_expr, all_fields);
                    generate_vec_read_with_length(element_type, &length_code)
                } else {
                    "unimplemented!(\"Vec reading without length not yet implemented\")".to_string()
                }
            } else if field_type.starts_with("PackableList<") {
                let element_type = &field_type[13..field_type.len() - 1];
                generate_packable_list_read(element_type)
            } else if field_type.starts_with("std::collections::HashMap<") {
                let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    if let Some(length_expr) = &field.length_expression {
                        let length_code = convert_length_expression(length_expr, all_fields);
                        generate_hashmap_read_with_length(key_type, value_type, &length_code, ctx)
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
                    generate_packable_hash_table_read(key_type, value_type, ctx)
                } else {
                    "unimplemented!(\"PackableHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PHashTable<") {
                let inner = &field_type["PHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    generate_phash_table_read(key_type, value_type)
                } else {
                    "unimplemented!(\"PHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if let Some(pos) = field_type.find('<') {
                let (type_name, generics) = field_type.split_at(pos);
                format!("{type_name}::{generics}::read(reader)")
            } else {
                format!("{field_type}::read(reader)")
            }
        }
    }
}

/// Generate a reader for a variant type (enum with switch)
fn generate_variant_reader_impl(
    ctx: &ReaderContext,
    _protocol_type: &ProtocolType,
    type_name: &str,
    field_set: &FieldSet,
) -> String {
    let mut out = String::new();

    let switch_field = field_set
        .switch_field
        .as_ref()
        .expect("Variant type must have switch field");
    let variant_fields = field_set
        .variant_fields
        .as_ref()
        .expect("Variant type must have variant fields");

    out.push_str(&format!("impl {} {{\n", type_name));
    out.push_str(
        "    pub fn read(reader: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {\n",
    );

    // Read all common fields
    for field in &field_set.common_fields {
        let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
        let read_call = generate_read_call(ctx, field, &field_set.common_fields);
        out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));

        // Generate subfield computations if any
        for subfield in &field.subfields {
            let subfield_name = safe_identifier(&subfield.name, IdentifierType::Field).name;
            let subfield_expr = convert_condition_expression(&subfield.value_expression, &field_set.common_fields);
            let subfield_rust_type = get_rust_type(&subfield.field_type);
            out.push_str(&format!("        let {} = ({}) as {};\n", subfield_name, subfield_expr, subfield_rust_type));
        }
    }

    // Generate match on switch field
    let switch_field_name = safe_identifier(switch_field, IdentifierType::Field).name;
    out.push_str(&format!("\n        match {} {{\n", switch_field_name));

    // Get the type of the switch field to see if it's an enum
    let switch_field_type = field_set
        .common_fields
        .iter()
        .find(|f| f.name == *switch_field)
        .map(|f| &f.field_type);

    // Group case values by field signature (same as type generator)
    // Map: field signature -> (primary_value, [all_values])
    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

    for (case_value, case_fields) in variant_fields {
        // Create a signature for these fields to group identical field sets
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

    // Generate each variant case (now grouped)
    for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
        // Sort values for consistent output
        all_values.sort();

        // Use the first sorted value to determine the variant name
        let first_value = all_values[0];
        let case_fields = variant_fields
            .get(&first_value)
            .expect("Field group must have fields");

        // Generate match patterns for ALL values in this group
        let mut case_patterns = Vec::new();
        for case_value in &all_values {
            // Convert case value to enum variant pattern if switch field is an enum
            let case_pattern = if let Some(switch_type) = switch_field_type {
                if ctx.enum_parent_map.contains_key(switch_type) {
                    // It's an enum - look up the enum variant name from the value
                    if let Some(variant_name) =
                        ctx.enum_value_map.get(&(switch_type.clone(), *case_value))
                    {
                        // Use the enum variant: EnumType::VariantName
                        format!("{}::{}", switch_type, variant_name)
                    } else {
                        // Fallback to formatted hex value if variant not found
                        format_hex_value(*case_value)
                    }
                } else {
                    format_hex_value(*case_value)
                }
            } else {
                format_hex_value(*case_value)
            };
            case_patterns.push(case_pattern);
        }

        // Join all patterns with |
        out.push_str(&format!(
            "            {} => {{\n",
            case_patterns.join(" | ")
        ));

        // Read variant-specific fields
        for field in case_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            // For variant fields, we need to look at all available fields (common + case)
            let mut all_fields = field_set.common_fields.clone();
            all_fields.extend(case_fields.clone());
            let read_call = generate_read_call(ctx, field, &all_fields);
            out.push_str(&format!(
                "                let {} = {}?;\n",
                field_name, read_call
            ));
        }

        // Construct the variant
        // Generate variant name from first case value (must match type generator)
        let variant_name = if first_value < 0 {
            // Negative value: -1 -> "TypeNeg1"
            format!("TypeNeg{}", first_value.abs())
        } else {
            // Positive value: format as hex and extract hex digits
            // 0x04 -> "04", 0xAB -> "AB"
            let hex_str = format!("{:X}", first_value);
            format!("Type{}", hex_str)
        };

        out.push('\n');
        out.push_str(&format!("                Ok(Self::{} {{\n", variant_name));

        // Add all common fields (except the switch field which is implicit in the variant)
        for field in &field_set.common_fields {
            if field.name != *switch_field {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("                    {},\n", field_name));
            }
        }

        // Add variant-specific fields
        for field in case_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("                    {},\n", field_name));
        }

        out.push_str("                })\n");
        out.push_str("            }\n");
    }

    // Add default case for unknown values
    // If switch field is an enum, cast it back to the underlying type for the error message
    let error_value = if let Some(switch_type) = switch_field_type {
        if let Some(parent_type) = ctx.enum_parent_map.get(switch_type) {
            let parent_rust_type = get_rust_type(parent_type);
            format!("{} as {}", switch_field_name, parent_rust_type)
        } else {
            switch_field_name.clone()
        }
    } else {
        switch_field_name.clone()
    };

    out.push_str(&format!(
        "            _ => Err(format!(\"Unknown {} value: {{:#x}}\", {}).into())\n",
        switch_field, error_value
    ));
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACDataType implementation
    out.push_str(&format!(
        "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn std::io::Read) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
        type_name, type_name
    ));

    out
}

/// Generate the appropriate read call for a given field
///
/// # Arguments
/// * `ctx` - Reader context with enum information
/// * `field` - The field to generate a read call for
/// * `all_fields` - All fields available in the current scope (used to resolve length expressions)
fn generate_read_call(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let field_type = &field.field_type;
    let is_optional = field.is_optional;
    let rust_type = get_rust_type(field_type);

    let base_read = match rust_type {
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
        _ => {
            // Check if it's an enum
            if let Some(parent_type) = ctx.enum_parent_map.get(field_type) {
                // It's an enum - read the parent type and cast
                let parent_rust_type = get_rust_type(parent_type);
                let read_fn = match parent_rust_type {
                    "u8" => "read_u8",
                    "i8" => "read_i8",
                    "u16" => "read_u16",
                    "i16" => "read_i16",
                    "u32" => "read_u32",
                    "i32" => "read_i32",
                    "u64" => "read_u64",
                    "i64" => "read_i64",
                    _ => panic!("Unsupported enum parent type: {}", parent_type),
                };
                format!("{}::try_from({}(reader)?)", field_type, read_fn)
            } else if field_type.starts_with("Vec<") {
                // Handle Vec - extract element type
                let element_type = &field_type[4..field_type.len() - 1];

                if let Some(length_expr) = &field.length_expression {
                    // Convert length expression to Rust code
                    let length_code = convert_length_expression(length_expr, all_fields);
                    generate_vec_read_with_length(element_type, &length_code)
                } else {
                    "unimplemented!(\"Vec reading without length not yet implemented\")".to_string()
                }
            } else if field_type.starts_with("PackableList<") {
                let element_type = &field_type[13..field_type.len() - 1];
                generate_packable_list_read(element_type)
            } else if field_type.starts_with("std::collections::HashMap<") {
                // Handle HashMap - extract key and value types
                let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();

                    if let Some(length_expr) = &field.length_expression {
                        // Convert length expression to Rust code
                        let length_code = convert_length_expression(length_expr, all_fields);
                        generate_hashmap_read_with_length(key_type, value_type, &length_code, ctx)
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
                    generate_packable_hash_table_read(key_type, value_type, ctx)
                } else {
                    "unimplemented!(\"PackableHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PHashTable<") {
                let inner = &field_type["PHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    generate_phash_table_read(key_type, value_type)
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
    };

    if is_optional {
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
                    format!("{}::{} as u32", parts[0], parts[1])
                } else {
                    mask_value.clone()
                }
            } else {
                mask_value.clone()
            };
            // Generate: if (flags & 0x8) != 0 { read().map(Some) } else { Ok(None) }
            format!(
                "if ({} & {}) != 0 {{ {}.map(Some) }} else {{ Ok(None) }}",
                mask_field_safe, mask_value_code, base_read
            )
        } else {
            // No condition - this shouldn't happen for truly optional fields, but handle it
            base_read
        }
    } else {
        base_read
    }
}

/// Convert a condition expression from XML format to Rust code
/// Examples: "RecordCount > 0" -> "record_count > 0"
fn convert_condition_expression(expr: &str, all_fields: &[Field]) -> String {
    let expr = expr.trim();
    let mut result = String::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        if ch.is_whitespace()
            || ch == '>'
            || ch == '<'
            || ch == '='
            || ch == '!'
            || ch == '&'
            || ch == '|'
            || ch == '('
            || ch == ')'
        {
            if !current_token.is_empty() {
                // Try to find a field with this name
                if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
                    let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    result.push_str(&safe_name);
                } else if current_token.chars().all(|c| c.is_numeric()) {
                    // It's a number
                    result.push_str(&current_token);
                } else {
                    // Unknown token - keep as-is but make it snake_case
                    let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
                    result.push_str(&safe_name);
                }
                current_token.clear();
            }

            if !ch.is_whitespace() {
                result.push(ch);
            } else if !result.is_empty() && !result.ends_with(' ') {
                result.push(' ');
            }
        } else {
            current_token.push(ch);
        }
    }

    // Handle any remaining token
    if !current_token.is_empty() {
        if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
            let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
            result.push_str(&safe_name);
        } else if current_token.chars().all(|c| c.is_numeric()) {
            result.push_str(&current_token);
        } else {
            let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
            result.push_str(&safe_name);
        }
    }

    result.trim().to_string()
}

/// Convert a length expression from XML format to Rust code
/// Examples: "Count" -> "count", "RecordCount - 1" -> "(record_count - 1)"
fn convert_length_expression(expr: &str, all_fields: &[Field]) -> String {
    // Simple expression parsing - handle basic arithmetic
    let expr = expr.trim();

    // Check if it's a simple field reference
    if let Some(field) = all_fields.iter().find(|f| f.name == expr) {
        let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
        return format!("{} as usize", safe_name);
    }

    // Handle arithmetic expressions (e.g., "RecordCount - 1")
    // Split on operators and convert field names
    let mut result = String::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        if ch.is_whitespace()
            || ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == '/'
            || ch == '('
            || ch == ')'
        {
            if !current_token.is_empty() {
                // Try to find a field with this name
                if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
                    let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    result.push_str(&format!("({} as usize)", safe_name));
                } else if current_token.chars().all(|c| c.is_numeric()) {
                    // It's a number
                    result.push_str(&current_token);
                } else {
                    // Unknown token - keep as-is but make it snake_case
                    let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
                    result.push_str(&format!("({} as usize)", safe_name));
                }
                current_token.clear();
            }

            if !ch.is_whitespace() {
                result.push(' ');
                result.push(ch);
                result.push(' ');
            }
        } else {
            current_token.push(ch);
        }
    }

    // Handle any remaining token
    if !current_token.is_empty() {
        if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
            let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
            result.push_str(&format!("({} as usize)", safe_name));
        } else if current_token.chars().all(|c| c.is_numeric()) {
            result.push_str(&current_token);
        } else {
            let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
            result.push_str(&format!("({} as usize)", safe_name));
        }
    }

    result.trim().to_string()
}

/// Generate a Vec read with a known length
fn generate_vec_read_with_length(element_type: &str, length_code: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let rust_type = get_rust_type(element_type);
    format!("read_vec::<{}>(reader, {})", rust_type, length_code)
}

/// Generate a PackableList read (count + vector of items)
fn generate_packable_list_read(element_type: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let rust_type = get_rust_type(element_type);
    format!("read_packable_list::<{}>(reader)", rust_type)
}

/// Generate a PackableHashTable read (count + max_size + table of items)
fn generate_packable_hash_table_read(
    key_type: &str,
    value_type: &str,
    ctx: &ReaderContext,
) -> String {
    // Helper to generate read code for a type
    let generate_simple_read = |typ: &str| -> String {
        let rust_type = get_rust_type(typ);
        match rust_type {
            "u8" => "read_u8(reader)?".to_string(),
            "i8" => "read_i8(reader)?".to_string(),
            "u16" => "read_u16(reader)?".to_string(),
            "i16" => "read_i16(reader)?".to_string(),
            "u32" => "read_u32(reader)?".to_string(),
            "i32" => "read_i32(reader)?".to_string(),
            "u64" => "read_u64(reader)?".to_string(),
            "i64" => "read_i64(reader)?".to_string(),
            "f32" => "read_f32(reader)?".to_string(),
            "f64" => "read_f64(reader)?".to_string(),
            "bool" => "read_bool(reader)?".to_string(),
            "String" => "read_string(reader)?".to_string(),
            _ => {
                if let Some(parent_type) = ctx.enum_parent_map.get(typ) {
                    let parent_rust_type = get_rust_type(parent_type);
                    let read_fn = match parent_rust_type {
                        "u8" => "read_u8",
                        "i8" => "read_i8",
                        "u16" => "read_u16",
                        "i16" => "read_i16",
                        "u32" => "read_u32",
                        "i32" => "read_i32",
                        "u64" => "read_u64",
                        "i64" => "read_i64",
                        _ => panic!("Unsupported enum parent type: {}", parent_type),
                    };
                    format!("{}::try_from({}(reader)?)?", typ, read_fn)
                } else {
                    format!("{}::read(reader)?", typ)
                }
            }
        }
    };

    let key_read = generate_simple_read(key_type);
    let value_read = generate_simple_read(value_type);

    // Call the read_packable_hash_table_with helper function
    format!(
        "read_packable_hash_table_with(reader, |r| {{
            {}
        }}, |r| {{
            {}
        }})",
        format!("Ok({})", key_read).replace("reader", "r"),
        format!("Ok({})", value_read).replace("reader", "r")
    )
}

/// Generate a PHashTable read (packed_size + table of items)
fn generate_phash_table_read(key_type: &str, value_type: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let key_rust = get_rust_type(key_type);
    let value_rust = get_rust_type(value_type);
    format!("read_phash_table::<{}, {}>(reader)", key_rust, value_rust)
}

/// Generate a HashMap read with a known length
fn generate_hashmap_read_with_length(
    key_type: &str,
    value_type: &str,
    length_code: &str,
    ctx: &ReaderContext,
) -> String {
    // Helper to generate read code for a type
    let generate_simple_read = |typ: &str| -> String {
        let rust_type = get_rust_type(typ);
        match rust_type {
            "u8" => "read_u8(reader)?".to_string(),
            "i8" => "read_i8(reader)?".to_string(),
            "u16" => "read_u16(reader)?".to_string(),
            "i16" => "read_i16(reader)?".to_string(),
            "u32" => "read_u32(reader)?".to_string(),
            "i32" => "read_i32(reader)?".to_string(),
            "u64" => "read_u64(reader)?".to_string(),
            "i64" => "read_i64(reader)?".to_string(),
            "f32" => "read_f32(reader)?".to_string(),
            "f64" => "read_f64(reader)?".to_string(),
            "bool" => "read_bool(reader)?".to_string(),
            "String" => "read_string(reader)?".to_string(),
            _ => {
                if let Some(parent_type) = ctx.enum_parent_map.get(typ) {
                    let parent_rust_type = get_rust_type(parent_type);
                    let read_fn = match parent_rust_type {
                        "u8" => "read_u8",
                        "i8" => "read_i8",
                        "u16" => "read_u16",
                        "i16" => "read_i16",
                        "u32" => "read_u32",
                        "i32" => "read_i32",
                        "u64" => "read_u64",
                        "i64" => "read_i64",
                        _ => panic!("Unsupported enum parent type: {}", parent_type),
                    };
                    format!("{}::try_from({}(reader)?)?", typ, read_fn)
                } else {
                    format!("{}::read(reader)?", typ)
                }
            }
        }
    };

    let key_read = generate_simple_read(key_type);
    let value_read = generate_simple_read(value_type);

    // The block is an expression that evaluates to Result<HashMap<K, V>, Error>
    // We use ? inside the block and wrap in Ok() at the end
    format!(
        "(|| -> Result<_, Box<dyn std::error::Error>> {{\n            let length = {};\n            let mut map = std::collections::HashMap::with_capacity(length);\n            for _ in 0..length {{\n                let key = {};\n                let value = {};\n                map.insert(key, value);\n            }}\n            Ok(map)\n        }})()",
        length_code, key_read, value_read
    )
}

pub fn generate(xml: &str, filter_types: &[String]) -> GeneratedCode {
    let ctx = GenerationContext::new(filter_types.to_vec());
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    let mut common_types: Vec<ProtocolType> = Vec::new();
    let mut c2s_types: Vec<ProtocolType> = Vec::new();
    let mut s2c_types: Vec<ProtocolType> = Vec::new();
    let mut enums: Vec<ProtocolEnum> = Vec::new();
    let mut current_type: Option<ProtocolType> = None;
    let mut current_enum: Option<ProtocolEnum> = None;
    let mut current_field_set: Option<FieldSet> = None;
    let mut current_direction = MessageDirection::None;

    // Track <if> and <maskmap> state
    let mut _in_if = false;

    // Context for field processing
    let mut field_ctx = FieldContext {
        in_switch: false,
        current_case_values: None,
        in_if_true: false,
        in_if_false: false,
        in_maskmap: false,
        if_true_fields: Vec::new(),
        if_false_fields: Vec::new(),
        current_if_condition: None,
        current_maskmap_field: None,
        current_mask_value: None,
        in_field: false,
        current_field: None,
    };

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
                    );
                } else if tag_name == "enum" {
                    process_enum_start_tag(&e, &mut current_enum);
                } else if tag_name == "field" {
                    // Start of a field tag with potential subfields
                    field_ctx.in_field = true;
                    field_ctx.current_field = create_field_from_tag(&e, &field_ctx);
                    debug!("Started field tag with potential subfields");
                } else if tag_name == "subfield" {
                    // Process subfield and add to current field
                    if let Some(subfield) = process_subfield_tag(&e) {
                        if let Some(ref mut field) = field_ctx.current_field {
                            field.subfields.push(subfield);
                            debug!("Added subfield to current field");
                        }
                    }
                } else if tag_name == "switch" {
                    field_ctx.in_switch = process_switch_tag(&e, &mut current_field_set);
                } else if tag_name == "case" {
                    field_ctx.current_case_values = process_case_tag(&e);
                } else if tag_name == "if" {
                    _in_if = true;
                    // Parse the test attribute
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"test" {
                            field_ctx.current_if_condition =
                                Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                    debug!(
                        "Entered <if> block with condition {:?}",
                        field_ctx.current_if_condition
                    );
                } else if tag_name == "true" {
                    field_ctx.in_if_true = true;
                    field_ctx.if_true_fields.clear();
                    debug!("Entered <true> block");
                } else if tag_name == "false" {
                    field_ctx.in_if_false = true;
                    field_ctx.if_false_fields.clear();
                    debug!("Entered <false> block");
                } else if tag_name == "maskmap" {
                    field_ctx.in_maskmap = true;
                    // Parse the name attribute (the field to check)
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            field_ctx.current_maskmap_field =
                                Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                    debug!(
                        "Entered <maskmap> block for field {:?}",
                        field_ctx.current_maskmap_field
                    );
                } else if tag_name == "mask" {
                    // Parse the value attribute (the bitmask value)
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            field_ctx.current_mask_value =
                                Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                    debug!(
                        "Entered <mask> block with value {:?}",
                        field_ctx.current_mask_value
                    );
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
                    );
                } else if tag_name == "field" {
                    process_field_tag(&e, &mut current_field_set, &mut field_ctx);
                } else if tag_name == "subfield" {
                    // Process subfield and add to current field (for Event::Empty tags)
                    if let Some(subfield) = process_subfield_tag(&e) {
                        if let Some(ref mut field) = field_ctx.current_field {
                            field.subfields.push(subfield);
                            debug!("Added subfield to current field (empty tag)");
                        }
                    }
                } else if tag_name == "vector" {
                    process_vector_tag(&e, &mut current_field_set, &mut field_ctx);
                } else if tag_name == "table" {
                    process_table_tag(&e, &mut current_field_set, &mut field_ctx);
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
                        debug!("DONE with type in {current_direction:?} section");
                    }
                    field_ctx.in_switch = false;
                    field_ctx.current_case_values = None;
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
                } else if e.name().as_ref() == b"field" {
                    // End of field tag - finalize and add the field
                    if field_ctx.in_field {
                        if let Some(field) = field_ctx.current_field.take() {
                            debug!("Finalizing field with {} subfields", field.subfields.len());

                            // If we're in an <if> block, collect fields separately
                            if field_ctx.in_if_true {
                                field_ctx.if_true_fields.push(field);
                                debug!("Added field to if_true_fields");
                            } else if field_ctx.in_if_false {
                                field_ctx.if_false_fields.push(field);
                                debug!("Added field to if_false_fields");
                            } else {
                                add_field_to_set(field, &mut current_field_set, &field_ctx);
                            }
                        }
                        field_ctx.in_field = false;
                    }
                } else if e.name().as_ref() == b"switch" {
                    field_ctx.in_switch = false;
                    debug!("Exited switch");
                } else if e.name().as_ref() == b"case" {
                    field_ctx.current_case_values = None;
                } else if e.name().as_ref() == b"if" {
                    // Merge if_true and if_false fields and add to current_field_set
                    let merged_fields = merge_if_fields(
                        std::mem::take(&mut field_ctx.if_true_fields),
                        std::mem::take(&mut field_ctx.if_false_fields),
                    );

                    if let Some(field_set) = &mut current_field_set {
                        for field in merged_fields {
                            if field_ctx.in_switch {
                                if let (Some(case_vals), Some(variant_fields)) = (
                                    &field_ctx.current_case_values,
                                    &mut field_set.variant_fields,
                                ) {
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

                    _in_if = false;
                    field_ctx.current_if_condition = None;
                    debug!("Exited <if> block");
                } else if e.name().as_ref() == b"true" {
                    field_ctx.in_if_true = false;
                    debug!("Exited <true> block");
                } else if e.name().as_ref() == b"false" {
                    field_ctx.in_if_false = false;
                    debug!("Exited <false> block");
                } else if e.name().as_ref() == b"maskmap" {
                    field_ctx.in_maskmap = false;
                    field_ctx.current_maskmap_field = None;
                    debug!("Exited <maskmap> block");
                } else if e.name().as_ref() == b"mask" {
                    field_ctx.current_mask_value = None;
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
    enums_out.push_str("use serde::{Serialize, Deserialize};\n");
    enums_out.push_str("use num_enum::TryFromPrimitive;\n\n");

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

    // Generate reader modules
    let readers_common =
        generate_readers_for_types(&ctx, &rectified_common_types, &enums, "common");
    let readers_c2s = generate_readers_for_types(&ctx, &rectified_c2s_types, &enums, "c2s");
    let readers_s2c = generate_readers_for_types(&ctx, &rectified_s2c_types, &enums, "s2c");

    GeneratedCode {
        enums: enums_out,
        common: common_out,
        c2s: generate_types_code(&rectified_c2s_types),
        s2c: generate_types_code(&rectified_s2c_types),
        readers_common,
        readers_c2s,
        readers_s2c,
    }
}
