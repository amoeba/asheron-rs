use crate::{
    identifiers::{IdentifierType, safe_identifier},
    types::{FieldSet, ProtocolType},
};

use crate::{generation::context::ReaderContext, generation::helpers};

/// Generate a reader for a simple struct (no variants)
pub fn generate_struct_reader_impl(
    ctx: &ReaderContext,
    _protocol_type: &ProtocolType,
    type_name: &str,
    field_set: &FieldSet,
) -> String {
    let mut out = String::new();

    // Add ACDataType implementation with the reading logic inlined
    out.push_str(&format!("impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n",
        type_name
    ));

    // Add struct-level tracing span
    out.push_str("        #[cfg(feature = \"tracing\")]\n");
    out.push_str(&format!("        let _span = tracing::span!(tracing::Level::DEBUG, \"read\", r#type = \"{}\").entered();\n\n", type_name));

    // Group consecutive fields with the same condition
    let field_groups = helpers::group_consecutive_fields_by_condition(&field_set.common_fields);

    // Generate reads for each group
    for group in &field_groups {
        helpers::generate_field_group_reads(
            ctx,
            type_name,
            &mut out,
            &group.condition,
            &group.fields,
            &field_set.common_fields,
        );
    }

    // Construct the struct
    out.push_str("\n        Ok(Self {\n");
    for field in &field_set.common_fields {
        // Skip alignment marker fields - they're only read, not stored
        if !field.name.starts_with("__alignment_marker_") {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("            {},\n", field_name));
        }
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}
