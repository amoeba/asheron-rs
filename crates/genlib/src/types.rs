use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Subfield {
    pub(crate) name: String,
    pub(crate) field_type: String,
    /// Expression to compute this subfield's value (e.g., "Size - 16" or "PackedSize & 0xFFFFFF")
    pub(crate) value_expression: String,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) field_type: String,
    /// Whether this field is optional (from <if> or <maskmap>)
    pub(crate) is_optional: bool,
    /// For Vec types, the length expression (e.g., "RecordCount - 1")
    pub(crate) length_expression: Option<String>,
    /// For optional fields, the condition under which they are present (e.g., "RecordCount > 0")
    pub(crate) optional_condition: Option<String>,
    /// For masked fields, the field name to check (e.g., "Flags")
    pub(crate) mask_field: Option<String>,
    /// For masked fields, the bitmask value (e.g., "0x8")
    pub(crate) mask_value: Option<String>,
    /// For if/true/false blocks: which branch(es) this field belongs to
    pub(crate) if_branch: Option<IfBranch>,
    /// For fields in both branches with different types: the original type from the false branch
    pub(crate) if_false_branch_type: Option<String>,
    /// Subfields computed from this field's value (e.g., extracting bits or doing arithmetic)
    pub(crate) subfields: Vec<Subfield>,
    /// For fields with nested switches (switch following this field)
    #[allow(dead_code)]
    pub(crate) nested_field_set: Option<Box<FieldSet>>,
}

/// Tracks which branch of an if/true/false block a field belongs to
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfBranch {
    /// Field is in the true branch only
    True,
    /// Field is in the false branch only
    False,
    /// Field exists in both branches but with different types (merged)
    Both,
}

#[derive(Debug, Clone)]
pub struct FieldSet {
    /// The field being switched on, if any (e.g., "Type")
    pub(crate) switch_field: Option<String>,
    /// Fields before the switch (or all fields if no switch)
    pub(crate) common_fields: Vec<Field>,
    /// Per-variant fields, keyed by case value (parsed as i64). None if no switch.
    pub(crate) variant_fields: Option<BTreeMap<i64, Vec<Field>>>,
    /// Nested switches within cases, keyed by case value (parsed as i64). None if no nested switches.
    pub(crate) nested_switches: Option<BTreeMap<i64, NestedSwitch>>,
}

#[derive(Debug, Clone)]
pub struct NestedSwitch {
    /// The name of the discriminator field for this nested switch (e.g., "TitleSource")
    pub(crate) switch_field: String,
    /// Fields that appear before the nested switch (within the case)
    pub(crate) common_fields: Vec<Field>,
    /// Fields after the nested switch (still within the case)
    pub(crate) trailing_fields: Vec<Field>,
    /// Per-variant fields for the nested switch, keyed by case value
    pub(crate) variant_fields: BTreeMap<i64, Vec<Field>>,
}

#[derive(Debug, Clone)]
pub struct EnumValue {
    pub(crate) name: String,
    pub(crate) value: i64, // Parsed numeric value for sorting and normalization
}

#[derive(Debug, Clone)]
pub struct ProtocolEnum {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) parent: String,
    pub(crate) values: Vec<EnumValue>,
    pub(crate) extra_derives: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ProtocolType {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) fields: Option<FieldSet>,
    /// If true, this is a primitive type (e.g., int, ushort) not a composite type
    pub(crate) is_primitive: bool,
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
    fn extract_hash_bounds_from_field(
        &self,
        field_type: &str,
        hash_params: &mut std::collections::HashSet<String>,
    ) {
        // Look for HashMap<K, V> patterns
        if field_type.starts_with("std::collections::HashMap<") {
            // Extract the key type from HashMap<K, V>
            let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
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
