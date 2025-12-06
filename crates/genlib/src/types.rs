use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) field_type: String,
}

#[derive(Debug)]
pub struct FieldSet {
    /// The field being switched on, if any (e.g., "Type")
    pub(crate) switch_field: Option<String>,
    /// Fields before the switch (or all fields if no switch)
    pub(crate) common_fields: Vec<Field>,
    /// Per-variant fields, keyed by case value. None if no switch.
    pub(crate) variant_fields: Option<BTreeMap<String, Vec<Field>>>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug)]
pub struct ProtocolEnum {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) parent: String,
    pub(crate) values: Vec<EnumValue>,
    pub(crate) is_mask: bool,
}

#[derive(Debug)]
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
}

impl ProtocolType {
    /// Check if this type supports the Eq trait derivation (i.e., has no float fields)
    pub fn supports_trait_eq(&self) -> bool {
        if let Some(ref field_set) = self.fields {
            !field_set.common_fields.iter().any(|field| {
                let rust_type = crate::get_rust_type(&field.field_type);
                rust_type == "f32" || rust_type == "f64"
            })
        } else {
            true
        }
    }
}
