use crate::type_utils::get_rust_type;

use super::types::GeneratedFile;

// Re-export functions from read_generation submodules
pub use super::read_generation::expression_readers::convert_length_expression;
pub use super::read_generation::primitive_readers::{
    convert_condition_expression, generate_read_call,
};
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
