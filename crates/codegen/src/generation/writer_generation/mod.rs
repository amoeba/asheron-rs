pub mod struct_writers;
pub mod variant_writers;

use crate::{
    identifiers::{IdentifierType, safe_identifier},
    type_utils::get_rust_type,
    types::ProtocolType,
};

/// Generate writer implementation for a single type (as an impl block on the type)
pub fn generate_writer_impl(
    ctx: &super::context::ReaderContext,
    protocol_type: &ProtocolType,
) -> String {
    let type_name = &protocol_type.name;
    let safe_type_name = safe_identifier(type_name, IdentifierType::Type);

    // Primitive types (u32, bool, etc.) have write_* helper functions
    if protocol_type.is_primitive {
        return String::new();
    }

    // Templated/generic types (PackableList<T>, PackableHashTable<T,U>, etc.)
    // use generic helper functions like write_packable_list() instead of impl blocks
    if protocol_type.templated.is_some() {
        return String::new();
    }

    // Handle newtype structs (parent type with no fields, but not C-style aliases)
    // These are generated as newtype structs and need a write() method
    if protocol_type.parent.is_some() && protocol_type.fields.is_none() {
        if let Some(parent_type) = &protocol_type.parent {
            // Special case: DataId has parent="PackedDWORD" in XML but is stored as u32
            // The packed format is only for writing, not storage
            let rust_type = if type_name == "DataId" && parent_type == "PackedDWORD" {
                "u32"
            } else {
                get_rust_type(parent_type)
            };

            if crate::type_utils::should_be_newtype_struct(&safe_type_name.name, rust_type) {
                // Generate write() for newtype struct
                // Strategy: Try to use a write_* helper function
                let write_call = match rust_type {
                    "u8" => "write_u8(writer, self.0)".to_string(),
                    "i8" => "write_i8(writer, self.0)".to_string(),
                    "u16" => "write_u16(writer, self.0)".to_string(),
                    "i16" => "write_i16(writer, self.0)".to_string(),
                    "u32" => "write_u32(writer, self.0)".to_string(),
                    "i32" => "write_i32(writer, self.0)".to_string(),
                    "u64" => "write_u64(writer, self.0)".to_string(),
                    "i64" => "write_i64(writer, self.0)".to_string(),
                    "f32" => "write_f32(writer, self.0)".to_string(),
                    "f64" => "write_f64(writer, self.0)".to_string(),
                    "bool" => "write_bool(writer, self.0)".to_string(),
                    // String or custom parent: call write_{lowercase_typename}()
                    _ => format!(
                        "write_{}(writer, &self.0)",
                        safe_type_name.name.to_lowercase()
                    ),
                };

                let impl_code = format!(
                    "impl {} {{\n    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {}?;\n        Ok(())\n    }}\n}}\n\n",
                    safe_type_name.name, write_call
                );
                let acwritable_code = format!(
                    "impl crate::writers::ACWritable for {} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {}::write(self, writer)\n    }}\n}}\n\n",
                    safe_type_name.name, safe_type_name.name
                );
                return format!("{}{}", impl_code, acwritable_code);
            }
        }
        // Type alias - doesn't need a writer
        return String::new();
    }

    let Some(field_set) = &protocol_type.fields else {
        // Empty struct - no fields to write
        // Special cases for variable-length packed types
        let (impl_code, acwritable_code) = match safe_type_name.name.as_str() {
            "PackedDWORD" => {
                let impl_code = format!(
                    "impl {} {{\n    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        Ok(())\n    }}\n}}\n\n",
                    safe_type_name.name
                );
                let acwritable_code = format!(
                    "impl crate::writers::ACWritable for {} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {}::write(self, writer)\n    }}\n}}\n\n",
                    safe_type_name.name, safe_type_name.name
                );
                (impl_code, acwritable_code)
            }
            "PackedWORD" => {
                let impl_code = format!(
                    "impl {} {{\n    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        Ok(())\n    }}\n}}\n\n",
                    safe_type_name.name
                );
                let acwritable_code = format!(
                    "impl crate::writers::ACWritable for {} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {}::write(self, writer)\n    }}\n}}\n\n",
                    safe_type_name.name, safe_type_name.name
                );
                (impl_code, acwritable_code)
            }
            _ => {
                let impl_code = format!(
                    "impl {} {{\n    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        Ok(())\n    }}\n}}\n\n",
                    safe_type_name.name
                );
                let acwritable_code = format!(
                    "impl crate::writers::ACWritable for {} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {}::write(self, writer)\n    }}\n}}\n\n",
                    safe_type_name.name, safe_type_name.name
                );
                (impl_code, acwritable_code)
            }
        };
        return format!("{}{}", impl_code, acwritable_code);
    };

    // Check if this is a variant type (has switch)
    if let Some(ref variant_fields) = field_set.variant_fields {
        // For variant types with standalone structs,
        // we generate writers for each variant struct instead of a monolithic writer.
        variant_writers::generate_variant_struct_writers(
            ctx,
            &safe_type_name.name,
            field_set,
            variant_fields,
        )
    } else {
        struct_writers::generate_struct_writer_impl(
            ctx,
            protocol_type,
            &safe_type_name.name,
            field_set,
        )
    }
}
