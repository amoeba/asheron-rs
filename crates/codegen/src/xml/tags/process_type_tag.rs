use log::debug;

use crate::types::{FieldSet, ProtocolType};

/// Process a type tag in the XML
pub fn process_type_tag(
    e: &quick_xml::events::BytesStart,
    is_self_closing: bool,
    types: &mut Vec<ProtocolType>,
    current_type: &mut Option<ProtocolType>,
    current_field_set: &mut Option<FieldSet>,
) {
    let mut name = None;
    let mut text = None;
    let mut is_primitive = false;
    let mut parent = None;
    let mut templated = None;
    let mut queue = None;

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
            b"templated" => {
                templated = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"queue" => {
                queue = Some(attr.unescape_value().unwrap().into_owned());
            }
            _ => {}
        }
    }

    if let Some(name) = name {
        let new_type = ProtocolType {
            name,
            text,
            fields: None,
            is_primitive,
            parent,
            templated,
            hash_bounds: Vec::new(),
            extra_derives: Vec::new(),
            category: crate::types::ProtocolCategory::Types, // Will be set properly during parsing
            queue,
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
                nested_switches: None,
            });
        }

        debug!("Processed type, is_self_closing={is_self_closing}");
    }
}
