use std::collections::BTreeMap;

use crate::{
    field_gen::build_derive_string,
    identifiers::{IdentifierType, safe_identifier},
    type_utils::{get_rust_type, should_be_newtype_struct},
    types::ProtocolType,
};

pub fn generate_type(protocol_type: &ProtocolType) -> String {
    let original_type_name = &protocol_type.name;

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
        // Special case: DataId has parent="PackedDWORD" in XML but C# reads it as uint32
        // The packed format is only for writing, not storage
        let rust_type = if type_name == "DataId" && parent_type == "PackedDWORD" {
            "u32"
        } else {
            get_rust_type(parent_type)
        };

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
                
                // Generate impl block for DataId to support reading from binary
                if type_name == "DataId" {
                    out.push_str("impl DataId {\n");
                    out.push_str("    pub fn read(reader: &mut dyn crate::readers::ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n");
                    out.push_str("        Ok(Self(crate::readers::read_u32(reader)?))\n");
                    out.push_str("    }\n");
                    out.push_str("}\n\n");
                    out.push_str("impl crate::readers::ACDataType for DataId {\n");
                    out.push_str("    fn read(reader: &mut dyn crate::readers::ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n");
                    out.push_str("        DataId::read(reader)\n");
                    out.push_str("    }\n");
                    out.push_str("}\n\n");
                }
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
        // First, generate all standalone variant structs
        let switch_field = field_set.switch_field.as_ref().unwrap();

        // Collect all case values from both variant_fields and nested_switches
        let mut all_case_values = std::collections::BTreeSet::new();
        for case_value in variant_fields.keys() {
            all_case_values.insert(*case_value);
        }
        if let Some(ref nested_switches) = field_set.nested_switches {
            for case_value in nested_switches.keys() {
                all_case_values.insert(*case_value);
            }
        }

        // Group case values by their field sets (to handle multi-value cases)
        let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

        for case_value in &all_case_values {
            let case_fields = variant_fields.get(case_value).cloned().unwrap_or_default();
            let mut field_sig = case_fields
                .iter()
                .map(|f| format!("{}:{}", f.name, f.field_type))
                .collect::<Vec<_>>()
                .join(";");

            // Include nested switch structure in signature to avoid grouping cases with different nested switches
            if let Some(ref nested_switches) = field_set.nested_switches
                && let Some(nested_switch) = nested_switches.get(case_value)
            {
                // Add nested switch discriminator and case values to signature
                let nested_sig = format!(
                    "__nested_{}_{:?}",
                    nested_switch.switch_field,
                    nested_switch.variant_fields.keys().collect::<Vec<_>>()
                );
                field_sig.push_str(&nested_sig);
            }

            field_groups
                .entry(field_sig)
                .or_insert_with(|| (*case_value, Vec::new()))
                .1
                .push(*case_value);
        }

        let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
        sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

        // Generate standalone variant structs for each variant
        for (_field_sig, (_primary_value, all_values)) in &sorted_groups {
            let mut sorted_values = all_values.clone();
            sorted_values.sort();
            let first_value = sorted_values[0];
            // Get case fields, or use empty vec if this case only has a nested switch
            let case_fields = variant_fields
                .get(&first_value)
                .cloned()
                .unwrap_or_default();
            out.push_str(&super::enum_generation::generate_variant_struct(
                type_name,
                first_value,
                field_set,
                switch_field,
                &case_fields,
            ));
        }

        // Now generate the main enum that references these structs
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

        for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
            all_values.sort();

            let first_value = all_values[0];

            let variant_name = super::helpers::generate_variant_name(first_value);

            let variant_struct_name =
                super::enum_generation::generate_variant_struct_name(type_name, first_value);

            // For tagged enums, don't use serde rename/alias since it breaks serialization
            // The variant name itself is sufficient for the tag

            // Use tuple variant that wraps the standalone struct
            out.push_str(&format!("    {variant_name}({variant_struct_name}),\n"));
        }

        out.push_str("}\n\n");
    } else {
        // Generate struct
        let mut field_out: Vec<String> = Vec::new();

        for field in &field_set.common_fields {
            // Skip alignment marker fields - they're only for reading
            if !field.name.starts_with("__alignment_marker_") {
                field_out.push(crate::field_gen::generate_field_line(field, false)); // false = is struct field
            }
        }

        let fields_out: String = field_out.join(",\n") + ",";

        let derives = build_derive_string(&protocol_type.extra_derives);

        // Add #[allow(dead_code)] for packet header structs that are infrastructure not public API
        let allow_dead_code = if type_name == "C2SPacket" || type_name == "S2CPacket" {
            "#[allow(dead_code)]\n"
        } else {
            ""
        };

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{allow_dead_code}{derives}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n"
            ));
        } else {
            out.push_str(&format!(
                "{allow_dead_code}{derives}\npub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n"
            ));
        }
    }

    out
}

pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

pub struct GeneratedCode {
    pub files: Vec<GeneratedFile>,
}
