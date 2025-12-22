use std::collections::BTreeMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    field_gen::{FieldContext, merge_if_fields},
    generation::GenerateSource,
    types::{FieldSet, NestedSwitch, ProtocolCategory, ProtocolEnum, ProtocolType},
    xml::{
        tags::{
            process_align_tag::process_align_tag, process_case_tag::process_case_tag,
            process_enum_start_tag::process_enum_start_tag,
            process_enum_value_tag::process_enum_value_tag, process_field_tag::process_field_tag,
            process_subfield_tag::process_subfield_tag, process_switch_tag::process_switch_tag,
            process_table_tag::process_table_tag, process_type_tag::process_type_tag,
            process_vector_tag::process_vector_tag,
        },
        utils::{create_field_from_tag::create_field_from_tag, route_field},
    },
};

/// Result type for XML parsing with named fields
pub struct ParseResult {
    pub enums: Vec<ProtocolEnum>,
    pub common_types: Vec<ProtocolType>,
    pub game_action_types: Vec<ProtocolType>,
    pub game_event_types: Vec<ProtocolType>,
    pub c2s_types: Vec<ProtocolType>,
    pub s2c_types: Vec<ProtocolType>,
    pub packet_types: Vec<ProtocolType>,
}

/// Parse XML content and extract protocol types and enums
pub fn parse_xml_content(xml: &str, source: GenerateSource) -> ParseResult {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();

    // Collections for each XML section
    let mut enum_types: Vec<ProtocolEnum> = Vec::new();
    let mut common_types: Vec<ProtocolType> = Vec::new();
    let mut game_action_types: Vec<ProtocolType> = Vec::new();
    let mut game_event_types: Vec<ProtocolType> = Vec::new();
    let mut c2s_types: Vec<ProtocolType> = Vec::new();
    let mut s2c_types: Vec<ProtocolType> = Vec::new();
    let mut packet_types: Vec<ProtocolType> = Vec::new();

    let mut current_type: Option<ProtocolType> = None;
    let mut current_enum: Option<ProtocolEnum> = None;
    let mut current_field_set: Option<FieldSet> = None;
    let mut current_direction = ProtocolCategory::None;

    // Track <if> and <maskmap> state
    let mut _in_if = false;

    // Context for field processing
    let mut field_ctx = FieldContext {
        in_switch: false,
        current_case_values: None,
        in_if_true: false,
        in_if_false: false,
        in_maskmap: false,
        if_true_fields: Vec::new(),
        if_false_fields: Vec::new(),
        current_if_condition: None,
        current_maskmap_field: None,
        current_mask_value: None,
        in_field: false,
        current_field: None,
        switch_nesting_level: 0,
        current_outer_case_value: None,
        nested_switch_common_fields: Vec::new(),
        nested_switch_trailing_fields: Vec::new(),
        nested_field_set: None,
        collecting_nested_trailing_fields: false,
    };

    loop {
        let event = reader.read_event_into(&mut buf);

        match event {
            Ok(Event::Start(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "enums" {
                    current_direction = ProtocolCategory::Enums;
                    debug!("Entered enums section");
                } else if tag_name == "types" {
                    current_direction = ProtocolCategory::Types;
                    debug!("Entered types section");
                } else if tag_name == "gameactions" {
                    current_direction = ProtocolCategory::GameActions;
                    debug!("Entered gameactions section");
                } else if tag_name == "gameevents" {
                    current_direction = ProtocolCategory::GameEvents;
                    debug!("Entered gameevents section");
                } else if tag_name == "c2s" {
                    current_direction = ProtocolCategory::C2S;
                    debug!("Entered c2s section");
                } else if tag_name == "s2c" {
                    current_direction = ProtocolCategory::S2C;
                    debug!("Entered s2c section");
                } else if tag_name == "packets" {
                    current_direction = match source {
                        GenerateSource::Protocol => ProtocolCategory::Packets,
                        GenerateSource::Network => ProtocolCategory::Network,
                    };
                    debug!("Entered packets section (source: {:?})", source);
                } else if tag_name == "type" {
                    let types_vec = match current_direction {
                        ProtocolCategory::Types => &mut common_types,
                        ProtocolCategory::GameActions => &mut game_action_types,
                        ProtocolCategory::GameEvents => &mut game_event_types,
                        ProtocolCategory::C2S => &mut c2s_types,
                        ProtocolCategory::S2C => &mut s2c_types,
                        ProtocolCategory::Packets | ProtocolCategory::Network => &mut packet_types,
                        ProtocolCategory::Enums | ProtocolCategory::None => &mut common_types,
                    };
                    process_type_tag(
                        &e,
                        false,
                        types_vec,
                        &mut current_type,
                        &mut current_field_set,
                    );
                } else if tag_name == "enum" {
                    process_enum_start_tag(&e, &mut current_enum);
                } else if tag_name == "field" {
                    // Start of a field tag with potential subfields
                    field_ctx.in_field = true;
                    field_ctx.current_field = create_field_from_tag(&e, &field_ctx);
                    debug!("Started field tag with potential subfields");
                } else if tag_name == "subfield" {
                    // Process subfield and add to current field
                    if let Some(subfield) = process_subfield_tag(&e)
                        && let Some(ref mut field) = field_ctx.current_field
                    {
                        field.subfields.push(subfield);
                        debug!("Added subfield to current field");
                    }
                } else if tag_name == "align" {
                    // Process alignment requirement - add alignment marker
                    if let Some(mut align_field) = process_align_tag(&e) {
                        // Mark as internal so it won't be added to struct definition
                        align_field.name = format!("__alignment_marker_{}", align_field.name);
                        route_field(align_field, &mut current_field_set, &mut field_ctx);
                        debug!("Added alignment field");
                    }
                } else if tag_name == "switch" {
                    if field_ctx.switch_nesting_level == 0 {
                        // Outer switch
                        field_ctx.in_switch = process_switch_tag(&e, &mut current_field_set);
                        field_ctx.switch_nesting_level = 1;
                    } else {
                        // Nested switch within a case
                        field_ctx.switch_nesting_level += 1;
                        let mut switch_name: Option<String> = None;
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                switch_name = Some(attr.unescape_value().unwrap().into_owned());
                            }
                        }
                        if let Some(name) = switch_name {
                            // Create a new FieldSet for this nested switch
                            let nested = FieldSet {
                                switch_field: Some(name),
                                common_fields: Vec::new(),
                                variant_fields: Some(BTreeMap::new()),
                                nested_switches: None,
                            };
                            field_ctx.nested_field_set = Some(nested);
                        }
                        debug!(
                            "Entered nested switch at level {}",
                            field_ctx.switch_nesting_level
                        );
                    }
                } else if tag_name == "case" {
                    field_ctx.current_case_values = process_case_tag(&e);
                    // If this is an outer-level case that might have a nested switch, track the case value
                    // Only set current_outer_case_value when we're at the outer switch level (level 1)
                    if field_ctx.switch_nesting_level == 1
                        && let Some(ref case_vals) = field_ctx.current_case_values
                        && !case_vals.is_empty()
                    {
                        field_ctx.current_outer_case_value = Some(case_vals[0]);
                    }
                } else if tag_name == "if" {
                    _in_if = true;
                    // Parse the test attribute
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"test" {
                            field_ctx.current_if_condition =
                                Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                    debug!(
                        "Entered <if> block with condition {:?}",
                        field_ctx.current_if_condition
                    );
                } else if tag_name == "true" {
                    field_ctx.in_if_true = true;
                    field_ctx.if_true_fields.clear();
                    debug!("Entered <true> block");
                } else if tag_name == "false" {
                    field_ctx.in_if_false = true;
                    field_ctx.if_false_fields.clear();
                    debug!("Entered <false> block");
                } else if tag_name == "maskmap" {
                    field_ctx.in_maskmap = true;
                    // Parse the name attribute (the field to check)
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            field_ctx.current_maskmap_field =
                                Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                    debug!(
                        "Entered <maskmap> block for field {:?}",
                        field_ctx.current_maskmap_field
                    );
                } else if tag_name == "mask" {
                    // Parse the value attribute (the bitmask value)
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            field_ctx.current_mask_value =
                                Some(attr.unescape_value().unwrap().into_owned());
                        }
                    }
                    debug!(
                        "Entered <mask> block with value {:?}",
                        field_ctx.current_mask_value
                    );
                }
            }
            Ok(Event::Empty(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    let types_vec = match current_direction {
                        ProtocolCategory::Types => &mut common_types,
                        ProtocolCategory::GameActions => &mut game_action_types,
                        ProtocolCategory::GameEvents => &mut game_event_types,
                        ProtocolCategory::C2S => &mut c2s_types,
                        ProtocolCategory::S2C => &mut s2c_types,
                        ProtocolCategory::Packets | ProtocolCategory::Network => &mut packet_types,
                        ProtocolCategory::Enums | ProtocolCategory::None => &mut common_types,
                    };
                    process_type_tag(
                        &e,
                        true,
                        types_vec,
                        &mut current_type,
                        &mut current_field_set,
                    );
                } else if tag_name == "field" {
                    process_field_tag(&e, &mut current_field_set, &mut field_ctx);
                } else if tag_name == "subfield" {
                    // Process subfield and add to current field (for Event::Empty tags)
                    if let Some(subfield) = process_subfield_tag(&e)
                        && let Some(ref mut field) = field_ctx.current_field
                    {
                        field.subfields.push(subfield);
                        debug!("Added subfield to current field (empty tag)");
                    }
                } else if tag_name == "vector" {
                    process_vector_tag(&e, &mut current_field_set, &mut field_ctx);
                } else if tag_name == "table" {
                    process_table_tag(&e, &mut current_field_set, &mut field_ctx);
                } else if tag_name == "align" {
                    // Process alignment requirement (as Empty event for self-closing tags)
                    if let Some(mut align_field) = process_align_tag(&e) {
                        // Mark as internal so it won't be added to struct definition
                        align_field.name = format!("__alignment_marker_{}", align_field.name);
                        route_field(align_field, &mut current_field_set, &mut field_ctx);
                        debug!("Added alignment field (empty tag)");
                    }
                } else if tag_name == "value" {
                    process_enum_value_tag(&e, &mut current_enum);
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"type" {
                    // Close out type
                    if let Some(mut ty) = current_type.take() {
                        ty.category = current_direction;
                        if !ty.is_primitive {
                            ty.fields = current_field_set.take();
                            // Extract hash bounds for HashMap key types
                            ty.extract_hash_bounds();
                        }
                        let types_vec = match current_direction {
                            ProtocolCategory::Types => &mut common_types,
                            ProtocolCategory::GameActions => &mut game_action_types,
                            ProtocolCategory::GameEvents => &mut game_event_types,
                            ProtocolCategory::C2S => &mut c2s_types,
                            ProtocolCategory::S2C => &mut s2c_types,
                            ProtocolCategory::Packets | ProtocolCategory::Network => {
                                &mut packet_types
                            }
                            ProtocolCategory::Enums | ProtocolCategory::None => &mut common_types,
                        };
                        types_vec.push(ty);
                        debug!("DONE with type in {} section", current_direction);
                    }
                    field_ctx.in_switch = false;
                    field_ctx.current_case_values = None;
                } else if e.name().as_ref() == b"enum" {
                    // Close out enum
                    if let Some(mut en) = current_enum.take() {
                        en.category = current_direction.as_non_none();
                        enum_types.push(en);
                    }
                } else if e.name().as_ref() == b"enums" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited enums section");
                } else if e.name().as_ref() == b"types" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited types section");
                } else if e.name().as_ref() == b"gameactions" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited gameactions section");
                } else if e.name().as_ref() == b"gameevents" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited gameevents section");
                } else if e.name().as_ref() == b"c2s" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited c2s section");
                } else if e.name().as_ref() == b"s2c" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited s2c section");
                } else if e.name().as_ref() == b"packets" {
                    current_direction = ProtocolCategory::None;
                    debug!("Exited packets section");
                } else if e.name().as_ref() == b"field" {
                    // End of field tag - finalize and add the field
                    if field_ctx.in_field {
                        if let Some(field) = field_ctx.current_field.take() {
                            debug!("Finalizing field with {} subfields", field.subfields.len());

                            // If we're in an <if> block, collect fields separately
                            if field_ctx.in_if_true {
                                field_ctx.if_true_fields.push(field);
                                debug!("Added field to if_true_fields");
                            } else if field_ctx.in_if_false {
                                field_ctx.if_false_fields.push(field);
                                debug!("Added field to if_false_fields");
                            } else if field_ctx.switch_nesting_level > 1 {
                                // In a nested switch - add to nested_field_set if available
                                if field_ctx.collecting_nested_trailing_fields {
                                    // Collecting fields after the nested switch ends
                                    field_ctx.nested_switch_trailing_fields.push(field);
                                    debug!("Added field to nested switch trailing fields");
                                } else if let Some(ref mut nested) = field_ctx.nested_field_set {
                                    if let Some(ref case_vals) = field_ctx.current_case_values {
                                        // We're in a nested case
                                        if let Some(variant_fields) = &mut nested.variant_fields {
                                            for case_val in case_vals {
                                                variant_fields
                                                    .entry(*case_val)
                                                    .or_insert_with(Vec::new)
                                                    .push(field.clone());
                                                debug!(
                                                    "Added field to nested switch case {case_val}"
                                                );
                                            }
                                        }
                                    } else {
                                        // Before the nested switch
                                        nested.common_fields.push(field.clone());
                                        field_ctx.nested_switch_common_fields.push(field);
                                        debug!("Added field to nested switch common fields");
                                    }
                                }
                            } else {
                                route_field(field, &mut current_field_set, &mut field_ctx);
                            }
                        }
                        field_ctx.in_field = false;
                    }
                } else if e.name().as_ref() == b"switch" {
                    if field_ctx.switch_nesting_level == 1 {
                        // Closing outer switch
                        field_ctx.in_switch = false;
                        field_ctx.switch_nesting_level = 0;
                        debug!("Exited outer switch");
                    } else if field_ctx.switch_nesting_level > 1 {
                        // Closing nested switch - mark that we're now collecting trailing fields
                        field_ctx.switch_nesting_level -= 1;
                        field_ctx.collecting_nested_trailing_fields = true;
                        debug!("Exited nested switch, now collecting trailing fields");
                    }
                } else if e.name().as_ref() == b"case" {
                    // If we have a nested switch that we were collecting trailing fields for, finalize it now
                    if field_ctx.collecting_nested_trailing_fields
                        && let (Some(outer_case), Some(nested)) = (
                            field_ctx.current_outer_case_value,
                            field_ctx.nested_field_set.take(),
                        )
                    {
                        if let Some(ref mut outer_field_set) = current_field_set {
                            if outer_field_set.nested_switches.is_none() {
                                outer_field_set.nested_switches = Some(BTreeMap::new());
                            }
                            if let Some(ref mut nested_switches) = outer_field_set.nested_switches {
                                let switch_obj = NestedSwitch {
                                    switch_field: nested.switch_field.clone().unwrap_or_default(),
                                    common_fields: field_ctx.nested_switch_common_fields.clone(),
                                    trailing_fields: field_ctx
                                        .nested_switch_trailing_fields
                                        .clone(),
                                    variant_fields: nested.variant_fields.unwrap_or_default(),
                                };
                                nested_switches.insert(outer_case, switch_obj);
                                debug!(
                                    "Stored nested switch for case {} with {} trailing fields",
                                    outer_case,
                                    field_ctx.nested_switch_trailing_fields.len()
                                );
                            }
                        }
                        field_ctx.nested_switch_common_fields.clear();
                        field_ctx.nested_switch_trailing_fields.clear();
                        field_ctx.collecting_nested_trailing_fields = false;
                    }
                    field_ctx.current_case_values = None;
                } else if e.name().as_ref() == b"if" {
                    // Merge if_true and if_false fields and add to current_field_set
                    let merged_fields = merge_if_fields(
                        std::mem::take(&mut field_ctx.if_true_fields),
                        std::mem::take(&mut field_ctx.if_false_fields),
                    );

                    if let Some(field_set) = &mut current_field_set {
                        for field in merged_fields {
                            if field_ctx.in_switch {
                                if let (Some(case_vals), Some(variant_fields)) = (
                                    &field_ctx.current_case_values,
                                    &mut field_set.variant_fields,
                                ) {
                                    // Add field to all current case values
                                    for case_val in case_vals {
                                        variant_fields
                                            .entry(*case_val)
                                            .or_insert_with(Vec::new)
                                            .push(field.clone());
                                    }
                                }
                            } else {
                                field_set.common_fields.push(field);
                            }
                        }
                    }

                    _in_if = false;
                    field_ctx.current_if_condition = None;
                    debug!("Exited <if> block");
                } else if e.name().as_ref() == b"true" {
                    field_ctx.in_if_true = false;
                    debug!("Exited <true> block");
                } else if e.name().as_ref() == b"false" {
                    field_ctx.in_if_false = false;
                    debug!("Exited <false> block");
                } else if e.name().as_ref() == b"maskmap" {
                    field_ctx.in_maskmap = false;
                    field_ctx.current_maskmap_field = None;
                    debug!("Exited <maskmap> block");
                } else if e.name().as_ref() == b"mask" {
                    field_ctx.current_mask_value = None;
                    debug!("Exited <mask> block");
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    ParseResult {
        enums: enum_types,
        common_types,
        game_action_types,
        game_event_types,
        c2s_types,
        s2c_types,
        packet_types,
    }
}
