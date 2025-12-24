use crate::type_utils::get_rust_type;

use super::types::GeneratedFile;

// Re-export functions from read_generation submodules
pub use super::read_generation::expression_readers::{
    convert_condition_expression, convert_length_expression,
};
pub use super::read_generation::primitive_readers::generate_read_call;
pub use super::read_generation::{
    ConditionKey, FieldGroup, generate_field_group_reads, group_consecutive_fields_by_condition,
};

/// Generate a module file (mod.rs) that declares and re-exports a list of modules
pub fn generate_module_file(modules: &[String], path: &str) -> GeneratedFile {
    let mut content = String::new();

    // Add pub mod declarations
    for module_name in modules {
        content.push_str(&format!("pub mod {};\n", module_name));
    }

    content.push('\n');

    // Add pub use re-exports (skip c2s_packet and s2c_packet as they're infrastructure not public API)
    for module_name in modules {
        if module_name != "c2s_packet" && module_name != "s2c_packet" {
            content.push_str(&format!("pub use {}::*;\n", module_name));
        }
    }

    GeneratedFile {
        path: path.to_string(),
        content,
    }
}

/// Get the reader function name for a given Rust type
pub fn get_reader_function_name(rust_type: &str) -> Option<&'static str> {
    match rust_type {
        "u8" => Some("read_u8"),
        "i8" => Some("read_i8"),
        "u16" => Some("read_u16"),
        "i16" => Some("read_i16"),
        "u32" => Some("read_u32"),
        "i32" => Some("read_i32"),
        "u64" => Some("read_u64"),
        "i64" => Some("read_i64"),
        "f32" => Some("read_f32"),
        "f64" => Some("read_f64"),
        _ => None,
    }
}

/// Get the writer function name for a given Rust type
pub fn get_writer_function_name(rust_type: &str) -> Option<&'static str> {
    match rust_type {
        "u8" => Some("write_u8"),
        "i8" => Some("write_i8"),
        "u16" => Some("write_u16"),
        "i16" => Some("write_i16"),
        "u32" => Some("write_u32"),
        "i32" => Some("write_i32"),
        "u64" => Some("write_u64"),
        "i64" => Some("write_i64"),
        "f32" => Some("write_f32"),
        "f64" => Some("write_f64"),
        _ => None,
    }
}

/// Generate a variant name from a case value
/// Negative values become TypeNeg{abs}, positive become Type{HEX}
pub fn generate_variant_name(value: i64) -> String {
    if value < 0 {
        format!("TypeNeg{}", value.abs())
    } else {
        format!("Type{:X}", value)
    }
}

/// Generate ACDataType implementation for enum/bitflag types
pub fn generate_acdata_type_impl(
    type_name: &str,
    parent_type: &str,
    conversion: &str, // Either "from_bits_retain" or "try_from"
) -> String {
    let rust_type = get_rust_type(parent_type);

    if let Some(read_fn) = get_reader_function_name(rust_type) {
        let conversion_expr = if conversion == "try_from" {
            format!("{type_name}::try_from(value)?")
        } else {
            format!("{type_name}::{conversion}(value)")
        };

        format!(
            "impl crate::readers::ACDataType for {type_name} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        let value = crate::readers::{read_fn}(reader)?;\n        Ok({conversion_expr})\n    }}\n}}\n\n"
        )
    } else {
        String::new()
    }
}

/// Generate ACWritable implementation for enum/bitflag types
pub fn generate_acwritable_impl(
    type_name: &str,
    parent_type: &str,
    is_bitflags: bool, // true for bitflags, false for regular enums
) -> String {
    let rust_type = get_rust_type(parent_type);

    if let Some(write_fn) = get_writer_function_name(rust_type) {
        let value_expr = if is_bitflags {
            // Bitflags: use .bits() to get the underlying value
            "self.bits()".to_string()
        } else {
            // Regular enum: clone and cast to the parent type
            // We need clone() because enums don't implement Copy
            format!("self.clone() as {}", rust_type)
        };

        format!(
            "impl crate::writers::ACWritable for {type_name} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        crate::writers::{write_fn}(writer, {value_expr})?;\n        Ok(())\n    }}\n}}\n\n"
        )
    } else {
        String::new()
    }
}

/// Group case values by their field signature
///
/// This is used to identify variant cases that have identical field structures,
/// allowing them to be represented by the same variant struct.
///
/// # Arguments
/// * `all_case_values` - All case values to group
/// * `variant_fields` - Map of case values to their fields
/// * `nested_switches` - Optional nested switch information
///
/// # Returns
/// A sorted vector of (field_signature, (primary_value, all_values)) tuples
pub fn group_case_values_by_field_signature(
    all_case_values: &[i64],
    variant_fields: &std::collections::BTreeMap<i64, Vec<crate::types::Field>>,
    nested_switches: &Option<std::collections::BTreeMap<i64, crate::types::NestedSwitch>>,
) -> Vec<(String, (i64, Vec<i64>))> {
    use std::collections::BTreeMap;

    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

    for case_value in all_case_values {
        let case_fields = variant_fields.get(case_value).cloned().unwrap_or_default();
        let mut field_sig = case_fields
            .iter()
            .map(|f| format!("{}:{}", f.name, f.field_type))
            .collect::<Vec<_>>()
            .join(";");

        // Include nested switch structure in signature to avoid grouping cases with different nested switches
        if let Some(nested_switches) = nested_switches
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
    sorted_groups
}
