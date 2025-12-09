use anyhow::Result;
use serde_json;
use std::env;
use std::fs::{self, File};
use std::io::{Cursor, Read};
use std::path::Path;

// Import enums from acprotocol
use acprotocol::enums::{C2SMessage, GameEvent, S2CMessage};

// Import specific message types we want to support
use acprotocol::gameactions::item_appraise::ItemAppraise;
use acprotocol::messages::s2c::communication_textbox_string::CommunicationTextboxString;
use acprotocol::messages::s2c::effects_play_script_type::EffectsPlayScriptType;
use acprotocol::messages::s2c::effects_sound_event::EffectsSoundEvent;
use acprotocol::messages::s2c::inventory_pickup_event::InventoryPickupEvent;
use acprotocol::messages::s2c::item_obj_desc_event::ItemObjDescEvent;
use acprotocol::gameevents::item_wear_item::ItemWearItem;
use acprotocol::gameevents::magic_dispel_enchantment::MagicDispelEnchantment;
use acprotocol::gameevents::magic_update_enchantment::MagicUpdateEnchantment;
use acprotocol::messages::s2c::qualities_private_update_attribute2nd_level::QualitiesPrivateUpdateAttribute2ndLevel;
use acprotocol::messages::s2c::qualities_update_instance_id::QualitiesUpdateInstanceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    C2S,
    S2C,
}

#[derive(Debug, Clone)]
struct PacketInfo {
    direction: Direction,
    frame_opcode: u32,
    message_opcode: u32,
    message_id: Option<u32>,
    name: String,
    c2s_message: Option<C2SMessage>,
    s2c_message: Option<S2CMessage>,
    game_event: Option<GameEvent>,
}

impl PacketInfo {
    fn from_filename(filename: &str) -> Result<Self> {
        // Parse filename like: "0000_ClientToServer_0xF7B1_0xF7B1_Item_Appraise.bin"
        let parts: Vec<&str> = filename.split('_').collect();
        if parts.len() < 5 {
            return Err(anyhow::anyhow!("Invalid filename format: {}", filename));
        }

        let direction = match parts[1] {
            "ClientToServer" => Direction::C2S,
            "ServerToClient" => Direction::S2C,
            _ => return Err(anyhow::anyhow!("Unknown direction: {}", parts[1])),
        };

        // Parse frame opcode as hex (e.g., "0xF7B0")
        let frame_opcode_str = parts[2];
        let frame_opcode =
            if frame_opcode_str.starts_with("0x") || frame_opcode_str.starts_with("0X") {
                u32::from_str_radix(&frame_opcode_str[2..], 16)?
            } else {
                frame_opcode_str.parse::<u32>()?
            };

        // Parse message opcode as hex (e.g., "0xF7B1" or "0x00C9")
        let message_opcode_str = parts[3];
        let message_opcode =
            if message_opcode_str.starts_with("0x") || message_opcode_str.starts_with("0X") {
                u32::from_str_radix(&message_opcode_str[2..], 16)?
            } else {
                message_opcode_str.parse::<u32>()?
            };

        // Extract the message name (everything after the message opcode, before .bin)
        // Skip: sequence, direction, frame_opcode, message_opcode
        let name = parts[4..].join("_").replace(".bin", "");

        // For now, no message_id extraction (can be added if needed)
        let message_id = None;

        // For matching, we need to know what type of message this is
        // Ordered messages: frame_opcode is 0xF7B0 (S2C) or 0xF7B1 (C2S), message_opcode is GameEvent
        // Direct messages: frame_opcode is the actual message opcode
        let c2s_message = if direction == Direction::C2S {
            C2SMessage::try_from(message_opcode).ok()
        } else {
            None
        };
        let s2c_message = if direction == Direction::S2C {
            S2CMessage::try_from(message_opcode).ok()
        } else {
            None
        };
        let game_event = if frame_opcode == 0xF7B0 || frame_opcode == 0xF7B1 {
            GameEvent::try_from(message_opcode).ok()
        } else {
            None
        };

        Ok(PacketInfo {
            direction,
            frame_opcode,
            message_opcode,
            message_id,
            name,
            c2s_message,
            s2c_message,
            game_event,
        })
    }
}

fn process_file(filename: &str) -> Result<()> {
    println!("Processing file: {}", filename);

    // Parse packet info from filename
    let packet_info = PacketInfo::from_filename(filename)?;
    println!("Packet info: {:?}", packet_info);

    // Read binary data
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    println!("Read {} bytes from {}", buffer.len(), filename);

    // Print hex dump for reference
    println!("Hex dump:");
    for (i, byte) in buffer.iter().enumerate() {
        if i % 16 == 0 {
            print!("{:04x}: ", i);
        }
        print!("{:02x} ", byte);
        if i % 16 == 15 {
            println!();
        }
    }
    if buffer.len() % 16 != 0 {
        println!();
    }

    // Ordered messages (0xF7B0/0xF7B1) have a 12-byte header before the message payload:
    // - OrderedObjectId (4 bytes)
    // - OrderedSequence (4 bytes)
    // - EventType (4 bytes)
    // Non-ordered messages may have the opcode at the start
    let mut wire_buffer = buffer.clone();
    let mut cursor = if packet_info.frame_opcode == 0xF7B0 || packet_info.frame_opcode == 0xF7B1 {
        println!(
            "Detected ordered message (frame_opcode: 0x{:04x}), skipping 12-byte header",
            packet_info.frame_opcode
        );
        let mut c = Cursor::new(&wire_buffer);
        c.set_position(12);
        c
    } else {
        // For non-ordered messages, the opcode may or may not be in the file
        // Check if the first 2 bytes match the message opcode
        let mut c = Cursor::new(&wire_buffer);
        if wire_buffer.len() >= 2 {
            use std::io::Read as StdRead;
            let mut opcode_bytes = [0u8; 2];
            wire_buffer.as_slice().read_exact(&mut opcode_bytes).ok();
            let file_opcode = u16::from_le_bytes(opcode_bytes) as u32;
            if file_opcode == packet_info.message_opcode {
                println!(
                    "Non-ordered message has opcode at start (0x{:04x}), skipping it",
                    file_opcode
                );
                c.set_position(2);
                c
            } else {
                println!(
                    "Non-ordered message, no opcode at start (expected 0x{:04x}, got 0x{:04x})",
                    packet_info.message_opcode, file_opcode
                );
                c
            }
        } else {
            println!("Non-ordered message too small to check for opcode");
            c
        }
    };

    // These files contain just the message payload, no packet headers
    // Try to parse message based on enum type
    let result = match packet_info.direction {
        Direction::C2S => match packet_info.c2s_message {
            Some(C2SMessage::OrderedGameAction) => {
                // Ordered messages use GameEvent for the actual payload
                match packet_info.game_event {
                    Some(GameEvent::ItemSetAppraiseInfo) => match ItemAppraise::read(&mut cursor) {
                        Ok(msg) => {
                            println!("Successfully parsed ItemAppraise (GameEvent):");
                            Some(serde_json::to_value(msg)?)
                        }
                        Err(e) => {
                            println!("Failed to parse ItemAppraise: {}", e);
                            None
                        }
                    },
                    _ => None,
                }
            }
            Some(msg) => {
                println!("Skipping unimplemented C2S message type: {:?}", msg);
                None
            }
            None => {
                println!("Unknown C2S opcode: 0x{:04X}", packet_info.message_opcode);
                None
            }
        },
        Direction::S2C => {
            // For ordered messages, check game_event first
            if packet_info.game_event.is_some() {
                match packet_info.game_event {
                    Some(GameEvent::MagicUpdateEnchantment) => {
                        match MagicUpdateEnchantment::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed MagicUpdateEnchantment:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse MagicUpdateEnchantment: {}", e);
                                None
                            }
                        }
                    }
                    Some(GameEvent::MagicDispelEnchantment) => {
                        match MagicDispelEnchantment::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed MagicDispelEnchantment:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse MagicDispelEnchantment: {}", e);
                                None
                            }
                        }
                    }
                    Some(GameEvent::ItemSetAppraiseInfo) => match ItemAppraise::read(&mut cursor) {
                        Ok(msg) => {
                            println!("Successfully parsed ItemSetAppraiseInfo (ItemAppraise):");
                            Some(serde_json::to_value(msg)?)
                        }
                        Err(e) => {
                            println!("Failed to parse ItemSetAppraiseInfo: {}", e);
                            None
                        }
                    },
                    Some(GameEvent::ItemServerSaysContainId) => {
                        match ItemWearItem::read(&mut cursor) {
                            Ok(msg) => {
                                println!(
                                    "Successfully parsed ItemServerSaysContainId (ItemWearItem):"
                                );
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse ItemServerSaysContainId: {}", e);
                                None
                            }
                        }
                    }
                    Some(GameEvent::ItemWearItem) => match ItemWearItem::read(&mut cursor) {
                        Ok(msg) => {
                            println!("Successfully parsed ItemWearItem:");
                            Some(serde_json::to_value(msg)?)
                        }
                        Err(e) => {
                            println!("Failed to parse ItemWearItem: {}", e);
                            None
                        }
                    },

                    Some(event) => {
                        println!("Skipping unimplemented GameEvent type: {:?}", event);
                        None
                    }
                    None => {
                        println!(
                            "Unknown GameEvent opcode: 0x{:04X}",
                            packet_info.message_opcode
                        );
                        None
                    }
                }
            } else {
                // Non-ordered messages
                match packet_info.s2c_message {
                    Some(S2CMessage::QualitiesPrivateUpdateAttribute2ndLevel) => {
                        match QualitiesPrivateUpdateAttribute2ndLevel::read(&mut cursor) {
                            Ok(msg) => {
                                println!(
                                    "Successfully parsed QualitiesPrivateUpdateAttribute2ndLevel:"
                                );
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!(
                                    "Failed to parse QualitiesPrivateUpdateAttribute2ndLevel: {}",
                                    e
                                );
                                None
                            }
                        }
                    }
                    Some(S2CMessage::QualitiesUpdateInstanceId) => {
                        match QualitiesUpdateInstanceId::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed QualitiesUpdateInstanceId:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse QualitiesUpdateInstanceId: {}", e);
                                None
                            }
                        }
                    }
                    Some(S2CMessage::ItemObjDescEvent) => {
                        match ItemObjDescEvent::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed ItemObjDescEvent:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse ItemObjDescEvent: {}", e);
                                None
                            }
                        }
                    }
                    Some(S2CMessage::CommunicationTextboxString) => {
                        match CommunicationTextboxString::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed CommunicationTextboxString:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse CommunicationTextboxString: {}", e);
                                None
                            }
                        }
                    }
                    Some(S2CMessage::InventoryPickupEvent) => {
                        match InventoryPickupEvent::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed InventoryPickupEvent:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse InventoryPickupEvent: {}", e);
                                None
                            }
                        }
                    }
                    Some(S2CMessage::EffectsPlayScriptType) => {
                        match EffectsPlayScriptType::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed EffectsPlayScriptType:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse EffectsPlayScriptType: {}", e);
                                None
                            }
                        }
                    }
                    Some(S2CMessage::EffectsSoundEvent) => {
                        match EffectsSoundEvent::read(&mut cursor) {
                            Ok(msg) => {
                                println!("Successfully parsed EffectsSoundEvent:");
                                Some(serde_json::to_value(msg)?)
                            }
                            Err(e) => {
                                println!("Failed to parse EffectsSoundEvent: {}", e);
                                None
                            }
                        }
                    }
                    Some(msg) => {
                        println!("Skipping unimplemented S2C message type: {:?}", msg);
                        None
                    }
                    None => {
                        println!("Unknown S2C opcode: 0x{:04X}", packet_info.message_opcode);
                        None
                    }
                }
            }
        }
    };

    // Print JSON if we successfully parsed
    if let Some(value) = result {
        match serde_json::to_string_pretty(&value) {
            Ok(json) => println!("JSON: {}", json),
            Err(e) => println!("Failed to serialize to JSON: {}", e),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <directory-or-file>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let path = Path::new(input_path);

    if !path.exists() {
        println!("Error: {} does not exist", input_path);
        std::process::exit(1);
    }

    let bin_files = if path.is_dir() {
        println!("Processing directory: {}", input_path);

        // Get all .bin files in the directory
        let entries = fs::read_dir(path)?;
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "bin") {
                if let Some(path_str) = entry_path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }

        if files.is_empty() {
            println!("No .bin files found in directory: {}", input_path);
            return Ok(());
        }

        println!("Found {} .bin files", files.len());
        files
    } else if path.is_file() && path.extension().map_or(false, |ext| ext == "bin") {
        println!("Processing single file: {}", input_path);
        vec![input_path.to_string()]
    } else {
        println!("Error: {} is not a directory or .bin file", input_path);
        std::process::exit(1);
    };

    // Process each file
    for filename in &bin_files {
        println!("\n{}", "=".repeat(80));
        if let Err(e) = process_file(filename) {
            println!("Error processing {}: {}", filename, e);
        }
    }

    println!("\nProcessed {} files", bin_files.len());
    Ok(())
}
