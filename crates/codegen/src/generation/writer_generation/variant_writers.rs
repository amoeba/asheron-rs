use std::collections::BTreeMap;

use crate::{
    identifiers::{IdentifierType, safe_identifier},
    types::{Field, FieldSet, NestedSwitch},
};

use crate::generation::{context::ReaderContext, enum_generation, helpers};

/// Generate writers for all variant structs of an enum type
pub fn generate_variant_struct_writers(
    ctx: &ReaderContext,
    type_name: &str,
    field_set: &FieldSet,
    variant_fields: &BTreeMap<i64, Vec<Field>>,
) -> String {
    let mut out = String::new();

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

    // Group case values by field signature (same as type generator)
    let all_case_values_vec: Vec<i64> = all_case_values.iter().copied().collect();
    let sorted_groups = helpers::group_case_values_by_field_signature(
        &all_case_values_vec,
        variant_fields,
        &field_set.nested_switches,
    );

    // Generate a writer for each variant struct
    for (_field_sig, (_primary_value, all_values)) in sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();
        let first_value = sorted_values[0];

        // Get case fields, or use empty vec if this case only has a nested switch
        let case_fields = variant_fields
            .get(&first_value)
            .cloned()
            .unwrap_or_default();
        let variant_struct_name =
            enum_generation::generate_variant_struct_name(type_name, first_value);
        out.push_str(&generate_variant_struct_writer_impl(
            ctx,
            type_name,
            &variant_struct_name,
            field_set,
            &case_fields,
            first_value,
        ));
    }

    // Generate a writer for the main enum that delegates to variant structs
    out.push_str(&generate_enum_writer_impl(
        ctx,
        type_name,
        field_set,
        variant_fields,
    ));

    out
}

/// Generate a writer for the main enum that delegates to variant struct writers
fn generate_enum_writer_impl(
    _ctx: &ReaderContext,
    type_name: &str,
    field_set: &FieldSet,
    variant_fields: &BTreeMap<i64, Vec<Field>>,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {type_name} {{\n"));
    out.push_str(
        "    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {\n",
    );

    // Add struct-level tracing span
    out.push_str("        #[cfg(feature = \"tracing\")]\n");
    out.push_str(&format!("        let _span = tracing::span!(tracing::Level::DEBUG, \"write\", r#type = \"{}\").entered();\n\n", type_name));

    // Don't write common fields here - they're included in the variant structs
    // The variant structs contain all fields (common + variant-specific)

    // Generate match on enum variants
    out.push_str("\n        match self {\n");

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

    // Group case values by field signature
    let all_case_values_vec: Vec<i64> = all_case_values.iter().copied().collect();
    let sorted_groups = helpers::group_case_values_by_field_signature(
        &all_case_values_vec,
        variant_fields,
        &field_set.nested_switches,
    );

    for (_field_sig, (_primary_value, all_values)) in sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();
        let first_value = sorted_values[0];

        let variant_struct_name =
            enum_generation::generate_variant_struct_name(type_name, first_value);

        // Generate variant name
        let variant_name = super::super::helpers::generate_variant_name(first_value);

        out.push_str(&format!(
            "            Self::{variant_name}(variant_struct) => {{\n"
        ));
        out.push_str(&format!(
            "                {variant_struct_name}::write(variant_struct, writer)?;\n"
        ));
        out.push_str("            },\n");
    }

    out.push_str("        }\n");
    out.push_str("        Ok(())\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACWritable implementation
    out.push_str(&format!(
        "impl crate::writers::ACWritable for {type_name} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {type_name}::write(self, writer)\n    }}\n}}\n\n"
    ));

    out
}

/// Generate a writer for a single variant struct
fn generate_variant_struct_writer_impl(
    ctx: &ReaderContext,
    type_name: &str,
    struct_name: &str,
    field_set: &FieldSet,
    case_fields: &[Field],
    case_value: i64,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {struct_name} {{\n"));

    // Check if this case has a nested switch and get its field name early
    let has_nested_switch = if let Some(ref nested_switches) = field_set.nested_switches {
        nested_switches.contains_key(&case_value)
    } else {
        false
    };

    // Get the nested switch field name if exists (as Option<String> for easier comparisons)
    let nested_switch_field_name = if has_nested_switch {
        field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .map(|ns| ns.switch_field.clone())
    } else {
        None
    };

    out.push_str("    #[allow(clippy::too_many_arguments)]\n");
    out.push_str("    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {\n");

    // Add struct-level tracing span
    out.push_str("        #[cfg(feature = \"tracing\")]\n");
    out.push_str(&format!("        let _span = tracing::span!(tracing::Level::DEBUG, \"write\", r#type = \"{}\").entered();\n\n", struct_name));

    // Write variant-specific fields
    for field in case_fields {
        // Skip the nested switch discriminator field - it's written by the nested enum
        let skip_field = nested_switch_field_name.as_ref() == Some(&field.name);
        if !skip_field {
            let mut all_fields = field_set.common_fields.clone();
            all_fields.extend(case_fields.iter().cloned());
            let write_call =
                crate::generation::write_generation::primitive_writers::generate_write_call(
                    ctx,
                    field,
                    &all_fields,
                );

            // write_call might be a multi-line block or single expression
            // Check if it already contains a semicolon or ends with '?'
            if write_call.contains('\n') || write_call.trim_end().ends_with('}') {
                // Multi-line block - use as-is
                out.push_str(&format!("        {}\n", write_call));
            } else {
                // Single-line expression - add semicolon
                out.push_str(&format!("        {};\n", write_call));
            }
        }
    }

    if has_nested_switch {
        let nested_switch = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();

        // Write common fields before nested switch
        for field in &nested_switch.common_fields {
            if field.name != nested_switch.switch_field {
                let mut all_fields = field_set.common_fields.clone();
                all_fields.extend(case_fields.iter().cloned());
                let write_call =
                    crate::generation::write_generation::primitive_writers::generate_write_call(
                        ctx,
                        field,
                        &all_fields,
                    );

                if write_call.contains('\n') || write_call.trim_end().ends_with('}') {
                    out.push_str(&format!("        {}\n", write_call));
                } else {
                    out.push_str(&format!("        {};\n", write_call));
                }
            }
        }

        // Write the nested switch enum
        let nested_enum_field_name =
            safe_identifier(&nested_switch.switch_field, IdentifierType::Field).name;
        out.push_str(&format!(
            "        self.{}.write(writer)?;\n",
            nested_enum_field_name
        ));

        // Write trailing fields
        for field in &nested_switch.trailing_fields {
            let mut all_fields = field_set.common_fields.clone();
            all_fields.extend(case_fields.iter().cloned());
            let write_call =
                crate::generation::write_generation::primitive_writers::generate_write_call(
                    ctx,
                    field,
                    &all_fields,
                );

            if write_call.contains('\n') || write_call.trim_end().ends_with('}') {
                out.push_str(&format!("        {}\n", write_call));
            } else {
                out.push_str(&format!("        {};\n", write_call));
            }
        }
    }

    out.push_str("        Ok(())\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate ACWritable for variant structs
    out.push_str(&format!(
        "impl crate::writers::ACWritable for {struct_name} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {struct_name}::write(self, writer)\n    }}\n}}\n\n"
    ));

    // Generate nested enum writer if this variant has a nested switch
    if has_nested_switch {
        let nested_switch = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();
        let nested_enum_name_raw =
            format!("{struct_name}{}{}", nested_switch.switch_field, "Variant");
        let nested_enum_name = safe_identifier(&nested_enum_name_raw, IdentifierType::Type).name;

        out.push_str(&generate_nested_switch_enum_writer(
            ctx,
            type_name,
            &nested_enum_name,
            nested_switch,
        ));
        out.push('\n');
    }

    out
}

/// Generate a writer for a nested switch enum
fn generate_nested_switch_enum_writer(
    ctx: &ReaderContext,
    _type_name: &str,
    enum_name: &str,
    nested_switch: &NestedSwitch,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {enum_name} {{\n"));
    out.push_str(
        "    pub fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {\n",
    );

    // Add struct-level tracing span
    out.push_str("        #[cfg(feature = \"tracing\")]\n");
    out.push_str(&format!("        let _span = tracing::span!(tracing::Level::DEBUG, \"write\", r#type = \"{}\").entered();\n\n", enum_name));

    // Match on enum variants and write the discriminator + fields
    out.push_str("        match self {\n");

    // Group nested case values by field signature
    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();
    for (case_value, case_fields) in &nested_switch.variant_fields {
        let field_sig = case_fields
            .iter()
            .map(|f| format!("{}:{}", f.name, f.field_type))
            .collect::<Vec<_>>()
            .join(";");

        field_groups
            .entry(field_sig)
            .or_insert_with(|| (*case_value, Vec::new()))
            .1
            .push(*case_value);
    }

    let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    for (_field_sig, (_primary_value, all_values)) in sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();

        let first_value = sorted_values[0];
        let case_fields = nested_switch
            .variant_fields
            .get(&first_value)
            .expect("Field group must have fields");

        // Generate variant name
        let variant_name = super::super::helpers::generate_variant_name(first_value);

        out.push_str(&format!(
            "            Self::{variant_name}(variant_struct) => {{\n",
        ));

        // Write the discriminator
        out.push_str(&format!(
            "                write_u8(writer, 0x{:02X})?;\n",
            first_value as u8
        ));

        // Write case fields
        for field in case_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            let write_call =
                crate::generation::write_generation::primitive_writers::generate_write_call(
                    ctx,
                    field,
                    case_fields,
                );

            // Replace field access pattern to use variant_struct
            let write_call = write_call.replace(
                &format!("self.{}", field_name),
                &format!("variant_struct.{}", field_name),
            );

            if write_call.contains('\n') || write_call.trim_end().ends_with('}') {
                out.push_str(&format!("                {}\n", write_call));
            } else {
                out.push_str(&format!("                {};\n", write_call));
            }
        }

        out.push_str("            },\n");
    }

    out.push_str("        }\n");
    out.push_str("        Ok(())\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACWritable implementation
    out.push_str(&format!(
        "impl crate::writers::ACWritable for {enum_name} {{\n    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {{\n        {enum_name}::write(self, writer)\n    }}\n}}\n\n"
    ));

    out
}
