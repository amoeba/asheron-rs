pub mod struct_readers;
pub mod variant_readers;

use crate::{
    field_gen::get_allow_unused_directive,
    identifiers::{IdentifierType, safe_identifier, to_snake_case},
    type_utils::get_rust_type,
    types::{Field, FieldSet, ProtocolType},
    util::format_hex_value,
};

/// Generate a single file containing both type definition and reader implementation
pub fn generate_type_and_reader_file(
    gen_ctx: &super::context::GenerationContext,
    reader_ctx: &super::context::ReaderContext,
    protocol_type: &ProtocolType,
) -> String {
    let mut out = String::new();

    // Add imports
    out.push_str("use serde::{Serialize, Deserialize};\n");
    out.push_str("use crate::readers::ACReader;\n");
    out.push_str("use crate::readers::*;\n");
    out.push_str("use crate::types::*;\n");
    out.push_str("use crate::enums::*;\n");
    out.push_str("use super::*;\n\n");

    // Generate type definition
    if protocol_type.is_primitive {
        let type_name = &protocol_type.name;
        let rust_type = get_rust_type(type_name);
        if rust_type != type_name {
            if let Some(ref text) = protocol_type.text {
                out.push_str(&format!("/// {text}\n"));
            }
            out.push_str(&format!(
                "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
            ));
        }
    } else {
        out.push_str(&super::type_generation::generate_type(protocol_type));
    }

    // Generate reader implementation if requested
    if gen_ctx.should_generate_reader(&protocol_type.name) {
        out.push_str(&generate_reader_impl(reader_ctx, protocol_type));
    }

    out
}

/// Generate a reader implementation for a single type (as an impl block on the type)
pub fn generate_reader_impl(
    ctx: &super::context::ReaderContext,
    protocol_type: &ProtocolType,
) -> String {
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

            if crate::type_utils::should_be_newtype_struct(&safe_type_name.name, rust_type) {
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
                    "impl {} {{\n    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        Ok(Self({}?))\n    }}\n}}\n\n",
                    safe_type_name.name, read_call
                );
                let acdatatype_code = format!(
                    "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
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
            "impl {} {{\n    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        Ok(Self {{}})\n    }}\n}}\n\n",
            safe_type_name.name
        );
        let acdatatype_code = format!(
            "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
            safe_type_name.name, safe_type_name.name
        );
        return format!("{}{}", impl_code, acdatatype_code);
    };

    // Check if this is a variant type (has switch)
    if let Some(ref variant_fields) = field_set.variant_fields {
        // For variant types with the new standalone struct approach,
        // we generate readers for each variant struct instead of a monolithic reader.
        variant_readers::generate_variant_struct_readers(
            ctx,
            &safe_type_name.name,
            field_set,
            variant_fields,
        )
    } else {
        struct_readers::generate_struct_reader_impl(
            ctx,
            protocol_type,
            &safe_type_name.name,
            field_set,
        )
    }
}
