use core::panic;
use std::collections::HashMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    types::{Field, FieldSet, ProtocolType},
    util::safe_field_name,
};

mod types;
mod util;

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
    let (safe_name, needs_rename) = safe_field_name(original_name);
    let rust_type = get_rust_type(&field.field_type);

    if needs_rename {
        format!("        #[serde(rename = \"{original_name}\")]\n        {safe_name}: {rust_type}")
    } else {
        format!("        {safe_name}: {rust_type}")
    }
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
            out.push_str(&format!("#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"));
        }
        return out;
    }

    let Some(field_set) = &protocol_type.fields else {
        // No fields, generate empty struct
        out.push_str(&format!(
            "#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct {type_name} {{}}\n\n"
        ));
        return out;
    };

    // Check if this is a variant type (has switch) or simple type
    if let Some(ref variant_fields) = field_set.variant_fields {
        // Generate enum
        let switch_field = field_set.switch_field.as_ref().unwrap();
        out.push_str(&format!(
            "#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = \"{switch_field}\")]
pub enum {type_name} {{\n"
        ));

        for (case_value, case_fields) in variant_fields {
            // Generate variant name from case value (e.g., "0x4" -> "Type4")
            let variant_name = if case_value.starts_with("0x") {
                format!("Type{}", &case_value[2..])
            } else {
                format!("Type{}", case_value)
            };

            out.push_str(&format!("    #[serde(rename = \"{case_value}\")]\n"));
            out.push_str(&format!("    {variant_name} {{\n"));

            // Add common fields first (excluding the switch field itself, as serde uses it as the tag)
            for field in &field_set.common_fields {
                if field.name != *switch_field {
                    out.push_str(&generate_field_line(field));
                    out.push_str(",\n");
                }
            }

            // Add variant-specific fields
            for field in case_fields {
                out.push_str(&generate_field_line(field));
                out.push_str(",\n");
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
            "#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        field_set.variant_fields = Some(HashMap::new());
        debug!("Entered switch, switch_field = {:?}", field_set.switch_field);
        true
    } else {
        false
    }
}

fn process_case_tag(e: &quick_xml::events::BytesStart) -> Option<String> {
    let mut value = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"value" {
            value = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    debug!("current_case_value is now {value:?}");
    value
}

fn process_field_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    in_switch: bool,
    current_case_value: &Option<String>,
) {
    let mut field_type = None;
    let mut field_name = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => {
                field_type = Some(attr.unescape_value().unwrap().into_owned())
            }
            b"name" => {
                field_name = Some(attr.unescape_value().unwrap().into_owned())
            }
            _ => {}
        }
    }

    debug!("Processing field {field_name:?}");

    if let (Some(fname), Some(ftype), Some(field_set)) =
        (field_name, field_type, current_field_set)
    {
        let new_field = Field {
            name: fname,
            field_type: ftype,
        };

        if in_switch {
            if let (Some(case_val), Some(variant_fields)) =
                (current_case_value, &mut field_set.variant_fields)
            {
                variant_fields
                    .entry(case_val.clone())
                    .or_insert_with(Vec::new)
                    .push(new_field);
                debug!("Added field to variant case {case_val}");
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
        let should_skip = filter_types.as_ref().map_or(false, |filters| !filters.contains(&name));

        if should_skip {
            debug!("Skipping type {name} because it's not in filter list.");
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

pub fn generate(xml: &str, filter_types: Option<Vec<String>>) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut out = String::new();

    // Add preamble with necessary imports
    out.push_str("use serde::{Serialize, Deserialize};\n\n");

    let mut types: Vec<ProtocolType> = Vec::new();
    let mut current_type: Option<ProtocolType> = None;
    let mut current_field_set: Option<FieldSet> = None;
    let mut current_case_value: Option<String> = None;
    let mut in_switch = false;

    loop {
        let event = reader.read_event_into(&mut buf);

        match event {
            Ok(Event::Start(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    process_type_tag(
                        &e,
                        false,
                        &mut types,
                        &mut current_type,
                        &mut current_field_set,
                        &filter_types,
                    );
                } else if tag_name == "field" {
                    process_field_tag(&e, &mut current_field_set, in_switch, &current_case_value);
} else if tag_name == "switch" {
                    in_switch = process_switch_tag(&e, &mut current_field_set);
                } else if tag_name == "case" {
                    current_case_value = process_case_tag(&e);
                }
            }
            Ok(Event::Empty(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    process_type_tag(
                        &e,
                        true,
                        &mut types,
                        &mut current_type,
                        &mut current_field_set,
                        &filter_types,
                    );
                } else if tag_name == "field" {
                    process_field_tag(&e, &mut current_field_set, in_switch, &current_case_value);
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"type" {
                    // Close out type
                    if let Some(mut ty) = current_type.take() {
                        if !ty.is_primitive {
                            ty.fields = current_field_set.take();
                        }
                        types.push(ty);
                        debug!("DONE: {types:?}");
                    }
                    in_switch = false;
                    current_case_value = None;
                } else if e.name().as_ref() == b"switch" {
                    in_switch = false;
                    debug!("Exited switch");
                } else if e.name().as_ref() == b"case" {
                    current_case_value = None;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    // Generate code for all types
    for protocol_type in &types {
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
}
