use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) field_type: String,
    /// Whether this field is optional (from <if> or <maskmap>)
    pub(crate) is_optional: bool,
}

#[derive(Debug, Clone)]
pub struct FieldSet {
    /// The field being switched on, if any (e.g., "Type")
    pub(crate) switch_field: Option<String>,
    /// Fields before the switch (or all fields if no switch)
    pub(crate) common_fields: Vec<Field>,
    /// Per-variant fields, keyed by case value. None if no switch.
    pub(crate) variant_fields: Option<BTreeMap<String, Vec<Field>>>,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug, Clone)]
pub struct ProtocolEnum {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) parent: String,
    pub(crate) values: Vec<EnumValue>,
    pub(crate) is_mask: bool,
    pub(crate) extra_derives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProtocolType {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) fields: Option<FieldSet>,
    /// If true, this is a primitive type (e.g., int, ushort) not a composite type
    pub(crate) is_primitive: bool,
    /// Optional Rust type mapping (e.g., "i32" for "int"). Used by map_type.
    pub(crate) rust_type: Option<String>,
    /// Parent type for type aliases (e.g., DWORD parent="uint")
    pub(crate) parent: Option<String>,
    /// Template parameters (e.g., "T" or "T,U" for generic types)
    pub(crate) templated: Option<String>,
    /// Generic parameters that need Eq + Hash bounds (for HashMap keys)
    pub(crate) hash_bounds: Vec<String>,
    /// Additional derives that this type needs (e.g., "Hash", "Eq", "PartialOrd")
    pub(crate) extra_derives: Vec<String>,
}

impl ProtocolType {
    /// Check if this type supports the Eq trait derivation (i.e., has no float fields)
    pub fn supports_trait_eq(&self) -> bool {
        if let Some(ref field_set) = self.fields {
            // Check common fields
            let has_float_in_common = field_set.common_fields.iter().any(|field| {
                let rust_type = crate::get_rust_type(&field.field_type);
                rust_type == "f32" || rust_type == "f64"
            });

            if has_float_in_common {
                return false;
            }

            // Check variant fields if they exist
            if let Some(ref variant_fields) = field_set.variant_fields {
                for case_fields in variant_fields.values() {
                    let has_float_in_variant = case_fields.iter().any(|field| {
                        let rust_type = crate::get_rust_type(&field.field_type);
                        rust_type == "f32" || rust_type == "f64"
                    });
                    if has_float_in_variant {
                        return false;
                    }
                }
            }

            true
        } else {
            true
        }
    }

    /// Extract generic parameter names that need Hash + Eq bounds from HashMap fields
    pub fn extract_hash_bounds(&mut self) {
        let mut hash_params = std::collections::HashSet::new();
        
        if let Some(ref field_set) = self.fields {
            // Check common fields for HashMap types
            for field in &field_set.common_fields {
                self.extract_hash_bounds_from_field(&field.field_type, &mut hash_params);
            }

            // Check variant fields for HashMap types
            if let Some(ref variant_fields) = field_set.variant_fields {
                for case_fields in variant_fields.values() {
                    for field in case_fields {
                        self.extract_hash_bounds_from_field(&field.field_type, &mut hash_params);
                    }
                }
            }
        }

        self.hash_bounds = hash_params.into_iter().collect();
    }

    /// Helper to extract hash bounds from a field type string
    fn extract_hash_bounds_from_field(&self, field_type: &str, hash_params: &mut std::collections::HashSet<String>) {
        // Look for HashMap<K, V> patterns
        if field_type.starts_with("std::collections::HashMap<") {
            // Extract the key type from HashMap<K, V>
            let inner = &field_type["std::collections::HashMap<".len()..field_type.len()-1];
            if let Some(comma_pos) = inner.find(',') {
                let key_type = inner[..comma_pos].trim();
                // If the key type is a single letter (like T, U, K, V), it's likely a generic parameter
                if key_type.len() == 1 && key_type.chars().next().unwrap().is_ascii_uppercase() {
                    hash_params.insert(key_type.to_string());
                }
            }
        }
    }
}
