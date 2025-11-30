use std::collections::HashMap;

use log::debug;
use quick_xml::{Reader, events::Event};

#[derive(Debug)]
struct Field {
    name: String,
    field_type: String,
}

#[derive(Debug)]
struct VariantFieldSet {
    name: String,
    fields: HashMap<u32, VariantFieldSet>,
}

#[derive(Debug)]
struct SchemaType {
    name: String,
    text: Option<String>,
    fields: Vec<Field>,
    variant: Option<VariantFieldSet>,
}

const RUST_RESERVED_WORDS: &[&str] = &["Self", "type"];

/// Convert PascalCase/camelCase to snake_case
fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            // Add underscore before uppercase if:
            // - Not the first character
            // - Previous character was lowercase or digit
            if i > 0 && (prev_is_lower || name.chars().nth(i - 1).is_some_and(|c| c.is_numeric())) {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_is_lower = false;
        } else {
            result.push(ch);
            prev_is_lower = ch.is_lowercase();
        }
    }

    result
}

/// Check if a field name is a Rust reserved word
fn is_reserved_word(name: &str) -> bool {
    RUST_RESERVED_WORDS.contains(&name)
}

/// Convert a field name to a safe Rust identifier in snake_case
fn safe_field_name(name: &str) -> (String, bool) {
    let snake_case = to_snake_case(name);

    if is_reserved_word(&snake_case) {
        let safe_name = format!("{snake_case}_");
        (safe_name, true)
    } else {
        let needs_rename = name != snake_case;
        (snake_case, needs_rename)
    }
}

fn generate_type(schema_type: &SchemaType) -> String {
    env_logger::init();

    let type_name = &schema_type.name;
    println!("generate_type: name = {type_name}");

    let mut out = String::new();

    if let Some(text_str) = &schema_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }

    let mut fields: Vec<String> = Vec::new();

    for field in &schema_type.fields {
        let original_name = &field.name;
        let (safe_name, needs_rename) = safe_field_name(original_name);

        if needs_rename {
            fields.push(format!(
                "    #[serde(rename = \"{original_name}\")]\n    {safe_name}: String"
            ));
        } else {
            fields.push(format!("    {safe_name}: String"));
        }
    }
    let fields_out = fields.join(",\n");

    out.push_str(
        format!(
            "#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct {type_name} {{
{fields_out}
}}\n\n",
        )
        .as_str(),
    );

    out
}

pub fn generate(xml: &str, filter_types: Option<Vec<String>>) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    // TODO: Use a better data structure?
    let mut out = String::new();

    let mut types: Vec<SchemaType> = Vec::new();
    let mut current_type: Option<SchemaType> = None;
    let mut current_variant_field_id: Option<String> = None;
    let mut current_variant_fieldset: Option<VariantFieldSet> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    let mut name = None;
                    let mut text = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
                            b"text" => {
                                text = Some(attr.unescape_value().unwrap().into_owned());
                            }
                            _ => {}
                        }
                    }

                    if let (Some(name)) = name {
                        // Skip type name based on active filters
                        if let Some(ref filters) = filter_types {
                            if !filters.contains(&name) {
                                debug!("Skipping type {name} because it's not in filter list.");
                                continue;
                            }
                        }
                        current_type = Some(SchemaType {
                            name,
                            text,
                            fields: Vec::new(),
                            variant: None,
                        });

                        debug!("current_type is now {current_type:?}");
                    }
                } else if tag_name == "field" {
                    if let Some(ref mut ty) = current_type {
                        let mut field_type = None;
                        let mut field_name = None;
                        let mut field_text = None;

                        for attr in e.attributes().flatten() {
                            match attr.key.as_ref() {
                                b"type" => {
                                    field_type = Some(attr.unescape_value().unwrap().into_owned())
                                }
                                b"name" => {
                                    field_name = Some(attr.unescape_value().unwrap().into_owned())
                                }
                                b"text" => {
                                    field_text = Some(attr.unescape_value().unwrap().into_owned());
                                }
                                _ => {}
                            }
                        }

                        if let (Some(fname), Some(ftype)) = (field_name, field_type) {
                            ty.fields.push(Field {
                                name: fname,
                                field_type: ftype,
                            });
                        }

                        debug!("after adding fields, current_type is now {current_type:?}");
                    }
                } else if tag_name == "switch" {
                    let mut name: Option<String> = None;

                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            name = Some(attr.unescape_value().unwrap().into_owned())
                        }
                    }

                    // Mark current type as variant so we start accumulating fields as variants
                    if let Some(name) = name {
                        current_variant_fieldset = Some(VariantFieldSet {
                            name,
                            fields: HashMap::new(),
                        });
                    }

                    debug!("current_variant_fieldset is now {current_variant_fieldset:?}");
                } else if tag_name == "case" {
                    let mut value = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"value" => value = Some(attr.unescape_value().unwrap().into_owned()),
                            _ => {}
                        }
                    }

                    debug!("TODO: Need to handle <case> element");
                    current_variant_field_id = value;
                    debug!("current_variant_field_id is now {current_variant_field_id:?}");
                }
            }
            Ok(Event::End(e)) => {
                // closing </type>
                if e.name().as_ref() == b"type" {
                    if let Some(ty) = current_type.take() {
                        types.push(ty);
                    }
                } else if e.name().as_ref() == b"switch" {
                    if let Some(ref mut ty) = current_type {
                        if let Some(v) = current_variant_fieldset.take() {
                            ty.variant = Some(v);
                        }
                    }
                } else if e.name().as_ref() == b"case" {
                    // TODO
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    for schema_type in types {
        out.push_str(&generate_type(&schema_type));
    }

    out
}
