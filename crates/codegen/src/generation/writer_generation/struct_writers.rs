use crate::types::{FieldSet, ProtocolType};

use crate::generation::context::ReaderContext;

/// Generate a writer for a simple struct (no variants)
pub fn generate_struct_writer_impl(
    ctx: &ReaderContext,
    _protocol_type: &ProtocolType,
    type_name: &str,
    field_set: &FieldSet,
) -> String {
    let mut out = String::new();

    // Add ACWritable implementation with the writing logic inlined
    out.push_str(&format!("impl crate::writers::ACWritable for {} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n",
        type_name
    ));

    // Add struct-level tracing span
    out.push_str("        #[cfg(feature = \"tracing\")]\n");
    out.push_str(&format!("        let _span = tracing::span!(tracing::Level::DEBUG, \"write\", r#type = \"{}\").entered();\n\n", type_name));

    // Generate writes for each field
    for field in &field_set.common_fields {
        // Skip alignment marker fields in struct definition, but process them for alignment
        if !field.name.starts_with("__alignment_marker_") {
            let write_call =
                crate::generation::write_generation::primitive_writers::generate_write_call(
                    ctx,
                    field,
                    &field_set.common_fields,
                );

            // The write_call might include its own conditional blocks for optional fields
            // Check if it's already a multi-line block (contains '\n')
            if write_call.contains('\n') {
                // Multi-line conditional write - use as-is
                out.push_str(&format!("        {}\n", write_call));
            } else {
                // Single-line write - use as-is
                out.push_str(&format!("        {};\n", write_call));
            }
        } else {
            // Generate alignment write
            let write_call =
                crate::generation::write_generation::primitive_writers::generate_write_call(
                    ctx,
                    field,
                    &field_set.common_fields,
                );
            out.push_str(&format!("        {};\n", write_call));
        }
    }

    out.push_str("        Ok(())\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}
