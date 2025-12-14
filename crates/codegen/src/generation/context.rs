use std::collections::{BTreeMap, HashSet};

use crate::types::{ProtocolEnum, ProtocolType};

/// Context for reader generation containing type information
pub struct ReaderContext {
    /// Map from enum name to its parent type (e.g., "NetAuthType" -> "uint")
    pub enum_parent_map: BTreeMap<String, String>,
    /// Map from (enum_name, value) to variant name (e.g., ("NetAuthType", 2) -> "AccountPassword")
    pub enum_value_map: BTreeMap<(String, i64), String>,
    /// Set of enum names that are bitflags (mask="true")
    pub mask_enums: HashSet<String>,
}

impl ReaderContext {
    pub fn new(
        enum_parent_map: BTreeMap<String, String>,
        enum_value_map: BTreeMap<(String, i64), String>,
        mask_enums: HashSet<String>,
    ) -> Self {
        Self {
            enum_parent_map,
            enum_value_map,
            mask_enums,
        }
    }
}

/// Analyze all types and add extra derives where needed
pub fn rectify_dependencies(
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
