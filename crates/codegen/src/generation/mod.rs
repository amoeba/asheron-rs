pub mod context;
pub mod enum_generation;
pub mod helpers;
pub mod read_generation;
pub mod reader_generation;
pub mod type_generation;
pub mod types;
pub mod unified_message_generation;

use std::collections::BTreeMap;

use crate::{identifiers::to_snake_case, type_utils::get_rust_type, types::ProtocolCategory};

pub use types::{GeneratedCode, GeneratedFile};

/// Represents the top-level protocol section a type/enum comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Indicates the source of the XML being parsed (protocol.xml vs network.xml)
pub enum GenerateSource {
    Protocol,
    Network,
}

pub fn generate(xml: &str, filter_types: &[String]) -> GeneratedCode {
    generate_with_source(xml, filter_types, GenerateSource::Protocol)
}

/// Helper function to perform code generation with files merging from multiple sources
/// This is used by both build.rs and xtask generate to ensure consistent code generation
pub fn generate_and_merge(
    protocol_xml: &str,
    network_xml: Option<&str>,
    filter_types: &[String],
) -> GeneratedCode {
    // Generate from protocol.xml
    let mut generated_code =
        generate_with_source(protocol_xml, filter_types, GenerateSource::Protocol);

    // Generate from network.xml if provided and merge results
    if let Some(network_xml) = network_xml {
        let network_code = generate_with_source(network_xml, filter_types, GenerateSource::Network);

        // Merge files from network.xml into generated_code
        for network_file in network_code.files {
            if network_file.path.ends_with("mod.rs") {
                // For mod.rs files, merge the pub mod and pub use declarations
                if let Some(existing) = generated_code
                    .files
                    .iter_mut()
                    .find(|f| f.path == network_file.path)
                {
                    // Merge module declarations from network.xml into existing mod.rs
                    let mut merged_content = existing.content.clone();

                    // Extract pub mod and pub use lines from network file
                    for line in network_file.content.lines() {
                        let trimmed = line.trim();
                        if (trimmed.starts_with("pub mod ") || trimmed.starts_with("pub use "))
                            && !merged_content.contains(trimmed)
                        {
                            // Add this line if it's not already present
                            merged_content.push('\n');
                            merged_content.push_str(line);
                        }
                    }

                    existing.content = merged_content;
                } else {
                    // No existing mod.rs, add network's mod.rs
                    generated_code.files.push(network_file);
                }
            } else {
                // For non-mod.rs files, just add/replace them
                if let Some(pos) = generated_code
                    .files
                    .iter()
                    .position(|f| f.path == network_file.path)
                {
                    generated_code.files[pos] = network_file;
                } else {
                    generated_code.files.push(network_file);
                }
            }
        }
    }

    generated_code
}

/// Generate code from protocol XML, with source indication for packets section
pub fn generate_with_source(
    xml: &str,
    filter_types: &[String],
    source: GenerateSource,
) -> GeneratedCode {
    let ctx = context::GenerationContext::new(filter_types.to_vec());

    // Parse XML content using the xml_parser module
    let (
        mut enum_types,
        common_types,
        game_action_types,
        game_event_types,
        c2s_types,
        s2c_types,
        packet_types,
    ) = crate::xml_parser::parse_xml_content(xml, source);

    // Rectify dependencies between types and enums
    let mut rectified_common_types = Vec::new();
    let mut rectified_c2s_types = Vec::new();
    let mut rectified_s2c_types = Vec::new();
    context::rectify_dependencies(
        &common_types,
        &c2s_types,
        &s2c_types,
        &mut enum_types,
        &mut rectified_common_types,
        &mut rectified_c2s_types,
        &mut rectified_s2c_types,
    );

    // Other type collections (no rectification needed for now, just clone)
    let rectified_game_action_types = game_action_types.clone();
    let rectified_game_event_types = game_event_types.clone();
    let rectified_packet_types = packet_types.clone();

    // Generate enums
    let mut enums_out = String::new();
    enums_out.push_str("use serde::{Serialize, Deserialize};\n");
    enums_out.push_str("use num_enum::TryFromPrimitive;\n");
    enums_out.push_str("use crate::readers::ACReader;\n\n");

    for protocol_enum in &enum_types {
        if protocol_enum.is_mask {
            enums_out.push_str(&enum_generation::generate_bitflags(protocol_enum));
        } else {
            enums_out.push_str(&enum_generation::generate_enum(protocol_enum));
        }
    }

    // Generate MessageQueue enum (only for protocol.xml, not network.xml)
    if source == GenerateSource::Protocol {
        enums_out.push_str(&enum_generation::generate_message_queue_enum(
            &rectified_c2s_types,
            &rectified_s2c_types,
            &game_action_types,
            &game_event_types,
        ));
    }

    // Generate common types - will add readers after building reader context
    let mut common_types_out = String::new();
    common_types_out.push_str("use serde::{Serialize, Deserialize};\n");
    common_types_out.push_str("use crate::readers::ACReader;\n");
    common_types_out.push_str("use crate::readers::*;\n");
    common_types_out.push_str("use crate::enums::*;\n\n");

    for protocol_type in &rectified_common_types {
        if protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let rust_type = get_rust_type(type_name);
            if rust_type != type_name {
                if let Some(ref text) = protocol_type.text {
                    common_types_out.push_str(&format!("/// {text}\n"));
                }
                common_types_out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
        } else {
            common_types_out.push_str(&type_generation::generate_type(protocol_type));
        }
    }

    // Build a map of enum names to their parent types for reader generation
    let enum_parent_map: BTreeMap<String, String> = enum_types
        .iter()
        .map(|e| (e.name.clone(), e.parent.clone()))
        .collect();

    // Build a map of (enum_name, value) -> variant_name for switch pattern matching
    let mut enum_value_map: BTreeMap<(String, i64), String> = BTreeMap::new();
    for protocol_enum in &enum_types {
        for enum_value in &protocol_enum.values {
            let safe_variant = crate::identifiers::safe_enum_variant_name(&enum_value.name);
            enum_value_map.insert(
                (protocol_enum.name.clone(), enum_value.value),
                safe_variant.name,
            );
        }
    }

    // Build a set of mask enum names (bitflags types)
    let mask_enums: std::collections::HashSet<String> = enum_types
        .iter()
        .filter(|e| e.is_mask)
        .map(|e| e.name.clone())
        .collect();

    let reader_ctx = context::ReaderContext::new(enum_parent_map, enum_value_map, mask_enums);

    // Add reader implementations to common types
    for protocol_type in &rectified_common_types {
        if ctx.should_generate_reader(&protocol_type.name) {
            common_types_out.push_str(&reader_generation::generate_reader_impl(
                &reader_ctx,
                protocol_type,
            ));
        }
    }

    // Generate individual files for each type
    let mut files = Vec::new();

    // Add enums file
    files.push(GeneratedFile {
        path: "enums/mod.rs".to_string(),
        content: enums_out,
    });

    // Generate common types as types/mod.rs
    files.push(GeneratedFile {
        path: "types/mod.rs".to_string(),
        content: common_types_out,
    });

    // Track module names for generating mod.rs files
    let mut c2s_modules = Vec::new();
    let mut s2c_modules = Vec::new();

    // Generate individual files for C2S messages
    for protocol_type in &rectified_c2s_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            // Remove underscores first, then convert to snake_case to avoid double underscores
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            c2s_modules.push(module_name.clone());
            let content =
                reader_generation::generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("messages/c2s/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for S2C messages
    for protocol_type in &rectified_s2c_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            // Remove underscores first, then convert to snake_case to avoid double underscores
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            s2c_modules.push(module_name.clone());
            let content =
                reader_generation::generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("messages/s2c/{}.rs", module_name),
                content,
            });
        }
    }

    // Track module names for each section
    let mut game_action_modules = Vec::new();
    let mut game_event_modules = Vec::new();
    let mut packet_modules = Vec::new();
    let mut network_modules = Vec::new();

    // Generate individual files for game actions
    for protocol_type in &rectified_game_action_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            game_action_modules.push(module_name.clone());
            let content =
                reader_generation::generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("gameactions/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for game events
    for protocol_type in &rectified_game_event_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            game_event_modules.push(module_name.clone());
            let content =
                reader_generation::generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("gameevents/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for packet types (from protocol.xml <packets>)
    // Skip Network category types - they go to network/ folder instead
    for protocol_type in &rectified_packet_types {
        if !protocol_type.is_primitive && protocol_type.category != ProtocolCategory::Network {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            packet_modules.push(module_name.clone());
            let content =
                reader_generation::generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("packets/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for network types (from network.xml <packets>)
    // These are identified by having category = Network
    for protocol_type in &rectified_packet_types {
        if !protocol_type.is_primitive && protocol_type.category == ProtocolCategory::Network {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            network_modules.push(module_name.clone());
            let content =
                reader_generation::generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("network/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate mod.rs files for all modules using helper function
    files.push(helpers::generate_module_file(
        &c2s_modules,
        "messages/c2s/mod.rs",
    ));
    files.push(helpers::generate_module_file(
        &s2c_modules,
        "messages/s2c/mod.rs",
    ));

    // Generate mod.rs for messages parent module
    let messages_mod = "pub mod c2s;\npub mod s2c;\n";
    files.push(GeneratedFile {
        path: "messages/mod.rs".to_string(),
        content: messages_mod.to_string(),
    });

    files.push(helpers::generate_module_file(
        &game_action_modules,
        "gameactions/mod.rs",
    ));
    files.push(helpers::generate_module_file(
        &game_event_modules,
        "gameevents/mod.rs",
    ));
    files.push(helpers::generate_module_file(
        &packet_modules,
        "packets/mod.rs",
    ));
    files.push(helpers::generate_module_file(
        &network_modules,
        "network/mod.rs",
    ));

    // Generate unified message types (only for protocol.xml, not network.xml)
    if source == GenerateSource::Protocol {
        // Generate unified message types
        let unified_message_content = unified_message_generation::generate_unified_message_types(
            &rectified_c2s_types,
            &rectified_s2c_types,
            &game_action_types,
            &game_event_types,
            &enum_types,
        );
        files.push(GeneratedFile {
            path: "unified.rs".to_string(),
            content: unified_message_content,
        });
    }

    // Generate root mod.rs for generated
    let generated_mod = "pub mod enums;\npub mod types;\npub mod messages;\npub mod gameactions;\npub mod gameevents;\npub mod packets;\npub mod network;\npub mod unified;\n";
    files.push(GeneratedFile {
        path: "mod.rs".to_string(),
        content: generated_mod.to_string(),
    });

    GeneratedCode { files }
}
