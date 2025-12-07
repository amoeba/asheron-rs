use core::panic;
use std::collections::BTreeMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    identifiers::{IdentifierType, safe_enum_variant_name, safe_identifier},
    types::{EnumValue, Field, FieldSet, ProtocolEnum, ProtocolType},
};

mod identifiers;
mod types;

/// Map an XML type name to a Rust type name.
pub fn get_rust_type(xml_type: &str) -> &str {
    match xml_type {
        "int" => "i32",
        "uint" => "u32",
        "short" => "i16",
        "ushort" => "u16",
        "long" => "i64",
        "ulong" => "u64",
        "byte" => "u8",
        "sbyte" => "i8",
        "float" => "f32",
        "double" => "f64",
        "bool" => "bool",
        "string" | "WString" => "String",
        // Keep other types as-is (custom types like ObjectId, Vector3, etc.)
        other => other,
    }
}

fn generate_field_line(field: &Field) -> String {
    let original_name = &field.name;
    let safe_id = safe_identifier(original_name, IdentifierType::Field);
    let rust_type = get_rust_type(&field.field_type);

    if safe_id.needs_rename {
        format!(
            "        #[serde(rename = \"{original_name}\")]\n        {}: {rust_type}",
            safe_id.name
        )
    } else {
        format!("        {}: {rust_type}", safe_id.name)
    }
}

fn generate_enum(protocol_enum: &ProtocolEnum) -> String {
    let enum_name = &protocol_enum.name;
    let mut out = String::new();

    if let Some(text_str) = &protocol_enum.text {
        out.push_str(&format!("/// {text_str}\n"));
    }

    // For mask enums, generate as a struct with bitflags
    if protocol_enum.is_mask {
        out.push_str(&format!(
            "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct {enum_name} {{\n        pub bits: {},\n}}\n\n",
            get_rust_type(&protocol_enum.parent)
        ));
    } else {
        // Generate regular enum
        out.push_str("#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\npub enum ");
        out.push_str(enum_name);
        out.push_str(" {\n");

        for enum_value in &protocol_enum.values {
            let original_name = &enum_value.name;

            // Generate variant name - convert to PascalCase if it has underscores
            let variant_name = if original_name.starts_with("0x") {
                format!("Type{}", &original_name[2..])
            } else {
                original_name.clone()
            };

            // Check if variant name is a reserved word or needs conversion
            let safe_variant = safe_enum_variant_name(&variant_name);

            // Determine if we need serde rename (if the safe name differs from original)
            let needs_serde_rename = safe_variant.name != *original_name;

            if enum_value.value.starts_with("0x") {
                // Hex value
                if needs_serde_rename {
                    out.push_str(&format!(
                        "    #[serde(rename = \"{}\")]\n    {} = {},\n",
                        original_name, safe_variant.name, enum_value.value
                    ));
                } else {
                    out.push_str(&format!(
                        "    {} = {},\n",
                        safe_variant.name, enum_value.value
                    ));
                }
            } else {
                // Decimal value
                if needs_serde_rename {
                    out.push_str(&format!(
                        "    #[serde(rename = \"{}\")]\n    {} = {},\n",
                        original_name, safe_variant.name, enum_value.value
                    ));
                } else {
                    out.push_str(&format!(
                        "    {} = {},\n",
                        safe_variant.name, enum_value.value
                    ));
                }
            }
        }

        out.push_str("}\n\n");
    }

    out
}

fn generate_type(protocol_type: &ProtocolType) -> String {
    let type_name = &protocol_type.name;
    println!("generate_type: name = {type_name}");

    let mut out = String::new();

    if let Some(text_str) = &protocol_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }

    // Handle parent types as type aliases
    if let Some(parent_type) = &protocol_type.parent {
        let rust_type = get_rust_type(parent_type);

        // Only generate alias if the rust type differs from the XML type name
        if rust_type != type_name {
            out.push_str(&format!(
                "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
            ));
        }
        return out;
    }

    let Some(field_set) = &protocol_type.fields else {
        // No fields, generate empty struct
        out.push_str(&format!(
            "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct {type_name} {{}}\n\n"
        ));
        return out;
    };

    // Check if this is a variant type (has switch) or simple type
    if let Some(ref variant_fields) = field_set.variant_fields {
        // Generate enum
        let switch_field = field_set.switch_field.as_ref().unwrap();

        out.push_str(&format!(
            "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = \"{switch_field}\")]
pub enum {type_name} {{\n"
        ));

        // Group case values by their field sets (to handle multi-value cases)
        // Map: field signature -> (primary_value, [all_values])
        let mut field_groups: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();

        for (case_value, case_fields) in variant_fields {
            // Create a signature for these fields to group identical field sets
            let field_sig = case_fields
                .iter()
                .map(|f| format!("{}:{}", f.name, f.field_type))
                .collect::<Vec<_>>()
                .join(";");

            field_groups
                .entry(field_sig)
                .or_insert_with(|| (case_value.clone(), Vec::new()))
                .1
                .push(case_value.clone());
        }

        // Sort by primary value for consistent output
        let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
        sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

        for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
            // Sort values for consistent output
            all_values.sort();

            // Use the first sorted value for both rename and variant name
            let first_value = &all_values[0];

            // Generate variant name from first sorted case value
            let variant_name = if first_value.starts_with("0x") || first_value.starts_with("0X") {
                // Hex value: "0x4" -> "Type4", "0xAB" -> "TypeAB"
                format!("Type{}", &first_value[2..].to_uppercase())
            } else if first_value.starts_with('-') {
                // Negative value: "-1" -> "TypeNeg1"
                format!("TypeNeg{}", &first_value[1..])
            } else {
                // Decimal value: "4" -> "Type4"
                format!("Type{}", first_value)
            };

            // Primary serde rename
            out.push_str(&format!("    #[serde(rename = \"{first_value}\")]\n"));

            // Add aliases for additional values (if multi-value case)
            for alias_value in &all_values[1..] {
                out.push_str(&format!("    #[serde(alias = \"{alias_value}\")]\n"));
            }

            out.push_str(&format!("    {variant_name} {{\n"));

            // Add common fields first (excluding the switch field itself, as serde uses it as the tag)
            for field in &field_set.common_fields {
                if field.name != *switch_field {
                    out.push_str(&generate_field_line(field));
                    out.push_str(",\n");
                }
            }

            // Add variant-specific fields (get from variant_fields using first_value)
            if let Some(case_fields) = variant_fields.get(first_value) {
                for field in case_fields {
                    out.push_str(&generate_field_line(field));
                    out.push_str(",\n");
                }
            }

            out.push_str("    },\n");
        }

        out.push_str("}\n\n");
    } else {
        // Generate struct
        let mut field_out: Vec<String> = Vec::new();

        for field in &field_set.common_fields {
            field_out.push(generate_field_line(field));
        }

        let fields_out: String = field_out.join(",\n");

        out.push_str(&format!(
            "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct {type_name} {{
{fields_out}
}}\n\n"
        ));
    }

    out
}

fn process_switch_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
) -> bool {
    let mut name: Option<String> = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"name" {
            name = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    if let (Some(switch_name), Some(field_set)) = (name, current_field_set) {
        field_set.switch_field = Some(switch_name);
        field_set.variant_fields = Some(BTreeMap::new());
        debug!(
            "Entered switch, switch_field = {:?}",
            field_set.switch_field
        );
        true
    } else {
        false
    }
}

fn process_case_tag(e: &quick_xml::events::BytesStart) -> Option<Vec<String>> {
    let mut value = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"value" {
            value = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    // Parse multi-value cases (e.g., "0x01 | 0x08 | 0x0A") into individual values
    let values = value.map(|v| {
        v.split('|')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
    });

    debug!("current_case_value is now {values:?}");
    values
}

fn process_enum_start_tag(
    e: &quick_xml::events::BytesStart,
    current_enum: &mut Option<ProtocolEnum>,
) {
    let mut name = None;
    let mut text = None;
    let mut parent = None;
    let mut is_mask = false;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"mask" => {
                is_mask = attr.unescape_value().unwrap() == "true";
            }
            _ => {}
        }
    }

    if let Some(name) = name {
        if let Some(parent) = parent {
            let new_enum = ProtocolEnum {
                name,
                text,
                parent,
                values: Vec::new(),
                is_mask,
            };
            *current_enum = Some(new_enum);
        }
    }
}

fn process_enum_value_tag(
    e: &quick_xml::events::BytesStart,
    current_enum: &mut Option<ProtocolEnum>,
) {
    let mut name = None;
    let mut value = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    if let (Some(name), Some(value), Some(current_enum)) = (name, value, current_enum) {
        // Handle multiple values in a single attribute (e.g., "0x0C | 0x0D")
        let values: Vec<&str> = value.split('|').collect();
        for val in values {
            let trimmed_val = val.trim();
            if !trimmed_val.is_empty() {
                let enum_value = EnumValue {
                    name: name.clone(),
                    value: trimmed_val.to_string(),
                };
                current_enum.values.push(enum_value);
            }
        }
    }
}

fn process_field_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    in_switch: bool,
    current_case_values: &Option<Vec<String>>,
) {
    let mut field_type = None;
    let mut field_name = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => field_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => field_name = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing field {field_name:?}");

    if let (Some(fname), Some(ftype), Some(field_set)) = (field_name, field_type, current_field_set)
    {
        let new_field = Field {
            name: fname,
            field_type: ftype,
        };

        if in_switch {
            if let (Some(case_vals), Some(variant_fields)) =
                (current_case_values, &mut field_set.variant_fields)
            {
                // Add the same field to all values in this case
                for case_val in case_vals {
                    variant_fields
                        .entry(case_val.clone())
                        .or_insert_with(Vec::new)
                        .push(new_field.clone());
                    debug!("Added field to variant case {case_val}");
                }
            }
        } else {
            field_set.common_fields.push(new_field);
            debug!("Added field to common_fields");
        }
    }
}

fn process_type_tag(
    e: &quick_xml::events::BytesStart,
    is_self_closing: bool,
    types: &mut Vec<ProtocolType>,
    current_type: &mut Option<ProtocolType>,
    current_field_set: &mut Option<FieldSet>,
    filter_types: &Option<Vec<String>>,
) {
    let mut name = None;
    let mut text = None;
    let mut is_primitive = false;
    let mut parent = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"primitive" => {
                is_primitive = attr.unescape_value().unwrap() == "true";
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            _ => {}
        }
    }

    if let Some(name) = name {
        let should_skip = filter_types
            .as_ref()
            .map_or(false, |filters| filters.contains(&name));

        if should_skip {
            debug!("Skipping type {name} because it's in exclusion list.");
            return;
        }

        let new_type = ProtocolType {
            name,
            text,
            fields: None,
            is_primitive,
            rust_type: None,
            parent,
        };

        // For self-closing tags, push immediately
        // For opening tags, set as current_type for further processing
        if is_self_closing {
            types.push(new_type);
        } else {
            *current_type = Some(new_type);
            *current_field_set = Some(FieldSet {
                switch_field: None,
                common_fields: Vec::new(),
                variant_fields: None,
            });
        }

        debug!("Processed type, is_self_closing={is_self_closing}");
    }
}

pub struct GeneratedCode {
    pub enums: String,
    pub common: String,
    pub c2s: String,
    pub s2c: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MessageDirection {
    None,
    C2S,            // <c2s> in messages section
    S2C,            // <s2c> in messages section
    GameActions,    // <gameactions> section (C2S)
    GameEvents,     // <gameevents> section (S2C)
}

pub fn generate(xml: &str, filter_types: Option<Vec<String>>) -> GeneratedCode {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    let mut common_types: Vec<ProtocolType> = Vec::new();
    let mut c2s_types: Vec<ProtocolType> = Vec::new();
    let mut s2c_types: Vec<ProtocolType> = Vec::new();
    let mut enums: Vec<ProtocolEnum> = Vec::new();
    let mut current_type: Option<ProtocolType> = None;
    let mut current_enum: Option<ProtocolEnum> = None;
    let mut current_field_set: Option<FieldSet> = None;
    let mut current_case_values: Option<Vec<String>> = None;
    let mut in_switch = false;
    let mut current_direction = MessageDirection::None;

    loop {
        let event = reader.read_event_into(&mut buf);

        match event {
            Ok(Event::Start(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "c2s" {
                    current_direction = MessageDirection::C2S;
                    debug!("Entered c2s section");
                } else if tag_name == "s2c" {
                    current_direction = MessageDirection::S2C;
                    debug!("Entered s2c section");
                } else if tag_name == "gameactions" {
                    current_direction = MessageDirection::GameActions;
                    debug!("Entered gameactions section");
                } else if tag_name == "gameevents" {
                    current_direction = MessageDirection::GameEvents;
                    debug!("Entered gameevents section");
                } else if tag_name == "type" {
                    let types_vec = match current_direction {
                        MessageDirection::C2S | MessageDirection::GameActions => &mut c2s_types,
                        MessageDirection::S2C | MessageDirection::GameEvents => &mut s2c_types,
                        MessageDirection::None => &mut common_types,
                    };
                    process_type_tag(
                        &e,
                        false,
                        types_vec,
                        &mut current_type,
                        &mut current_field_set,
                        &filter_types,
                    );
                } else if tag_name == "enum" {
                    process_enum_start_tag(&e, &mut current_enum);
                } else if tag_name == "field" {
                    process_field_tag(&e, &mut current_field_set, in_switch, &current_case_values);
                } else if tag_name == "switch" {
                    in_switch = process_switch_tag(&e, &mut current_field_set);
                } else if tag_name == "case" {
                    current_case_values = process_case_tag(&e);
                }
            }
            Ok(Event::Empty(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    let types_vec = match current_direction {
                        MessageDirection::C2S | MessageDirection::GameActions => &mut c2s_types,
                        MessageDirection::S2C | MessageDirection::GameEvents => &mut s2c_types,
                        MessageDirection::None => &mut common_types,
                    };
                    process_type_tag(
                        &e,
                        true,
                        types_vec,
                        &mut current_type,
                        &mut current_field_set,
                        &filter_types,
                    );
                } else if tag_name == "field" {
                    process_field_tag(&e, &mut current_field_set, in_switch, &current_case_values);
                } else if tag_name == "value" {
                    process_enum_value_tag(&e, &mut current_enum);
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"type" {
                    // Close out type
                    if let Some(mut ty) = current_type.take() {
                        if !ty.is_primitive {
                            ty.fields = current_field_set.take();
                        }
                        let types_vec = match current_direction {
                            MessageDirection::C2S | MessageDirection::GameActions => &mut c2s_types,
                            MessageDirection::S2C | MessageDirection::GameEvents => &mut s2c_types,
                            MessageDirection::None => &mut common_types,
                        };
                        types_vec.push(ty);
                        debug!("DONE with type in {:?} section", current_direction);
                    }
                    in_switch = false;
                    current_case_values = None;
                } else if e.name().as_ref() == b"enum" {
                    // Close out enum
                    if let Some(en) = current_enum.take() {
                        enums.push(en);
                    }
                } else if e.name().as_ref() == b"c2s" {
                    current_direction = MessageDirection::None;
                    debug!("Exited c2s section");
                } else if e.name().as_ref() == b"s2c" {
                    current_direction = MessageDirection::None;
                    debug!("Exited s2c section");
                } else if e.name().as_ref() == b"gameactions" {
                    current_direction = MessageDirection::None;
                    debug!("Exited gameactions section");
                } else if e.name().as_ref() == b"gameevents" {
                    current_direction = MessageDirection::None;
                    debug!("Exited gameevents section");
                } else if e.name().as_ref() == b"switch" {
                    in_switch = false;
                    debug!("Exited switch");
                } else if e.name().as_ref() == b"case" {
                    current_case_values = None;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    // Helper function to generate code for a list of types
    let generate_types_code = |types: &Vec<ProtocolType>| -> String {
        let mut out = String::new();
        out.push_str("use serde::{Serialize, Deserialize};\n\n");

        for protocol_type in types {
            if protocol_type.is_primitive {
                // Generate type alias for primitive types
                let type_name = &protocol_type.name;
                let rust_type = get_rust_type(type_name);

                // Only generate alias if the rust type differs from the XML type name
                if rust_type != type_name {
                    if let Some(ref text) = protocol_type.text {
                        out.push_str(&format!("/// {text}\n"));
                    }
                    out.push_str(&format!(
                        "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                    ));
                }
            } else {
                out.push_str(&generate_type(protocol_type));
            }
        }
        out
    };

    // Generate enums
    let mut enums_out = String::new();
    enums_out.push_str("use serde::{Serialize, Deserialize};\n\n");

    for protocol_enum in &enums {
        enums_out.push_str(&generate_enum(protocol_enum));
    }

    // Generate common types
    let mut common_out = String::new();
    common_out.push_str("use serde::{Serialize, Deserialize};\n\n");

    for protocol_type in &common_types {
        if protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let rust_type = get_rust_type(type_name);
            if rust_type != type_name {
                if let Some(ref text) = protocol_type.text {
                    common_out.push_str(&format!("/// {text}\n"));
                }
                common_out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
        } else {
            common_out.push_str(&generate_type(protocol_type));
        }
    }

    GeneratedCode {
        enums: enums_out,
        common: common_out,
        c2s: generate_types_code(&c2s_types),
        s2c: generate_types_code(&s2c_types),
    }
}
