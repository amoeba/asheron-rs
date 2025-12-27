/// Generate message types: Message, MessageKind, C2SMessage, S2CMessage
/// with improved enum structure that contains message data as variants
use crate::identifiers::ProtocolIdentifier;
use crate::types::{ProtocolEnum, ProtocolType};

/// Generate the message module with Message, MessageKind, C2SMessage, S2CMessage enums
pub fn generate_message_types(
    c2s_types: &[ProtocolType],
    s2c_types: &[ProtocolType],
    game_action_types: &[ProtocolType],
    game_event_types: &[ProtocolType],
    enums: &[ProtocolEnum],
) -> String {
    let mut out = String::new();

    // Add imports
    out.push_str("use serde::{Serialize, Deserialize};\n");
    out.push_str("use crate::readers::*;\n");
    out.push_str("use crate::enums::*;\n");
    out.push_str("use crate::writers::{ACWritable, ACWriter, write_u32, write_item};\n");
    out.push_str("use crate::messages::c2s;\n");
    out.push_str("use crate::messages::s2c;\n");
    out.push_str("use crate::gameactions;\n");
    out.push_str("use crate::gameevents;\n\n");

    // Generate Direction enum
    out.push_str("/// Message direction\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]\n");
    out.push_str("pub enum Direction {\n");
    out.push_str("    ClientToServer,\n");
    out.push_str("    ServerToClient,\n");
    out.push_str("}\n\n");

    // Generate MessageKind enum
    out.push_str("/// The main message kind enum\n");
    out.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    out.push_str("pub enum MessageKind {\n");
    out.push_str("    /// Client to Server messages\n");
    out.push_str("    C2S(Box<C2SMessage>),\n");
    out.push_str("    /// Server to Client messages\n");
    out.push_str("    S2C(Box<S2CMessage>),\n");
    out.push_str("}\n\n");

    // Generate MessageKind::read implementation
    out.push_str("impl MessageKind {\n");
    out.push_str("    pub fn read(reader: &mut dyn ACReader, direction: Direction) -> Result<Self, Box<dyn std::error::Error>> {\n");
    out.push_str("        match direction {\n");
    out.push_str("            Direction::ClientToServer => Ok(MessageKind::C2S(Box::new(C2SMessage::read(reader)?))),\n");
    out.push_str("            Direction::ServerToClient => Ok(MessageKind::S2C(Box::new(S2CMessage::read(reader)?))),\n");
    out.push_str("        }\n");
    out.push_str("    }\n\n");
    out.push_str("    pub fn queue(&self) -> Option<MessageQueue> {\n");
    out.push_str("        match self {\n");
    out.push_str("            MessageKind::C2S(msg) => msg.queue(),\n");
    out.push_str("            MessageKind::S2C(msg) => msg.queue(),\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Find the opcode enums
    let c2s_enum = enums.iter().find(|e| e.name == "C2SMessage");
    let s2c_enum = enums.iter().find(|e| e.name == "S2CMessage");
    let game_action_enum = enums.iter().find(|e| e.name == "GameAction");
    let game_event_enum = enums.iter().find(|e| e.name == "GameEvent");

    // Generate GameActionMessage enum (for OrderedGameAction sub-messages)
    out.push_str(&generate_message_enum(
        "GameActionMessage",
        game_action_types,
        "gameactions",
        game_action_enum,
    ));

    // Generate GameEventMessage enum (for OrderedGameEvent sub-messages)
    out.push_str(&generate_message_enum(
        "GameEventMessage",
        game_event_types,
        "gameevents",
        game_event_enum,
    ));

    // Generate C2SMessage enum with variants containing message data
    out.push_str(&generate_c2s_message_enum(c2s_types, c2s_enum));

    // Generate S2CMessage enum with variants containing message data
    out.push_str(&generate_s2c_message_enum(s2c_types, s2c_enum));

    out
}

/// Generate C2SMessage enum with special handling for OrderedGameAction
fn generate_c2s_message_enum(
    message_types: &[ProtocolType],
    opcode_enum: Option<&ProtocolEnum>,
) -> String {
    let mut out = String::new();

    // Generate enum
    out.push_str("/// C2SMessage enum with message data\n");
    out.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    out.push_str("pub enum C2SMessage {\n");

    for protocol_type in message_types {
        if !protocol_type.is_primitive {
            let name = ProtocolIdentifier::new(&protocol_type.name);

            out.push_str(&format!(
                "    {}(c2s::{}),\n",
                name.no_underscores(),
                name.no_underscores()
            ));
        }
    }

    // Add OrderedGameAction variant (doesn't have a message type def, only enum value)
    out.push_str("    OrderedGameAction {\n");
    out.push_str("        sequence: u32,\n");
    out.push_str("        action: GameActionMessage,\n");
    out.push_str("    },\n");

    out.push_str("}\n\n");

    // Generate read implementation
    out.push_str("impl C2SMessage {\n");
    out.push_str("    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n");
    out.push_str("        let opcode = read_u32(reader)?;\n");
    out.push_str("        let opcode_enum = crate::enums::C2SMessage::try_from(opcode)\n");
    out.push_str(
        "            .map_err(|_| format!(\"Unknown C2SMessage opcode: 0x{:04X}\", opcode))?;\n\n",
    );
    out.push_str("        match opcode_enum {\n");

    if let Some(protocol_enum) = opcode_enum {
        for protocol_type in message_types {
            if !protocol_type.is_primitive {
                let name = ProtocolIdentifier::new(&protocol_type.name);

                if let Some(_enum_value) = protocol_enum
                    .values
                    .iter()
                    .find(|v| v.name == name.original())
                {
                    out.push_str(&format!(
                        "            crate::enums::C2SMessage::{} => Ok(C2SMessage::{}(c2s::{}::read(reader)?)),\n",
                        name.no_underscores(), name.no_underscores(), name.no_underscores()
                    ));
                }
            }
        }

        // Special handling for OrderedGameAction
        out.push_str("            crate::enums::C2SMessage::OrderedGameAction => {\n");
        out.push_str("                let sequence = read_u32(reader)?;\n");
        out.push_str("                let action = GameActionMessage::read(reader)?;\n");
        out.push_str("                Ok(C2SMessage::OrderedGameAction { sequence, action })\n");
        out.push_str("            }\n");
    }

    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // Generate queue method
    out.push_str("    pub fn queue(&self) -> Option<MessageQueue> {\n");
    out.push_str("        match self {\n");

    for t in message_types {
        if let Some(ref queue) = t.queue {
            let name = ProtocolIdentifier::new(&t.name);
            let queue_name = ProtocolIdentifier::new(queue);
            out.push_str(&format!(
                "            C2SMessage::{}(_) => Some(MessageQueue::{}),\n",
                name.no_underscores(),
                queue_name.pascal_case()
            ));
        }
    }

    out.push_str("            C2SMessage::OrderedGameAction { action, .. } => action.queue(),\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate ACWritable implementation
    out.push_str("impl ACWritable for C2SMessage {\n");
    out.push_str("    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {\n");
    out.push_str("        match self {\n");

    for protocol_type in message_types {
        if !protocol_type.is_primitive {
            let name = ProtocolIdentifier::new(&protocol_type.name);
            out.push_str(&format!(
                "            C2SMessage::{}(msg) => {{\n",
                name.no_underscores()
            ));
            out.push_str(&format!(
                "                write_u32(writer, crate::enums::C2SMessage::{} as u32)?;\n",
                name.no_underscores()
            ));
            out.push_str("                write_item(writer, msg)?;\n");
            out.push_str("                Ok(())\n");
            out.push_str("            }\n");
        }
    }

    out.push_str("            C2SMessage::OrderedGameAction { sequence, action } => {\n");
    out.push_str("                write_u32(writer, crate::enums::C2SMessage::OrderedGameAction as u32)?;\n");
    out.push_str("                write_u32(writer, *sequence)?;\n");
    out.push_str("                write_item(writer, action)?;\n");
    out.push_str("                Ok(())\n");
    out.push_str("            }\n");

    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}

/// Generate S2CMessage enum with special handling for OrderedGameEvent
fn generate_s2c_message_enum(
    message_types: &[ProtocolType],
    opcode_enum: Option<&ProtocolEnum>,
) -> String {
    let mut out = String::new();

    // Generate enum
    out.push_str("/// S2CMessage enum with message data\n");
    out.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    out.push_str("pub enum S2CMessage {\n");

    for protocol_type in message_types {
        if !protocol_type.is_primitive {
            let name = ProtocolIdentifier::new(&protocol_type.name);

            out.push_str(&format!(
                "    {}(s2c::{}),\n",
                name.no_underscores(),
                name.no_underscores()
            ));
        }
    }

    // Add OrderedGameEvent variant (doesn't have a message type def, only enum value)
    out.push_str("    OrderedGameEvent {\n");
    out.push_str("        object_id: u32,\n");
    out.push_str("        sequence: u32,\n");
    out.push_str("        event: Box<GameEventMessage>,\n");
    out.push_str("    },\n");

    out.push_str("}\n\n");

    // Generate read implementation
    out.push_str("impl S2CMessage {\n");
    out.push_str("    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n");
    out.push_str("        let opcode = read_u32(reader)?;\n");
    out.push_str("        let opcode_enum = crate::enums::S2CMessage::try_from(opcode)\n");
    out.push_str(
        "            .map_err(|_| format!(\"Unknown S2CMessage opcode: 0x{:04X}\", opcode))?;\n\n",
    );
    out.push_str("        match opcode_enum {\n");

    if let Some(protocol_enum) = opcode_enum {
        for protocol_type in message_types {
            if !protocol_type.is_primitive {
                let name = ProtocolIdentifier::new(&protocol_type.name);

                if let Some(_enum_value) = protocol_enum
                    .values
                    .iter()
                    .find(|v| v.name == name.original())
                {
                    out.push_str(&format!(
                        "            crate::enums::S2CMessage::{} => Ok(S2CMessage::{}(s2c::{}::read(reader)?)),\n",
                        name.no_underscores(), name.no_underscores(), name.no_underscores()
                    ));
                }
            }
        }

        // Special handling for OrderedGameEvent
        out.push_str("            crate::enums::S2CMessage::OrderedGameEvent => {\n");
        out.push_str("                let object_id = read_u32(reader)?;\n");
        out.push_str("                let sequence = read_u32(reader)?;\n");
        out.push_str("                let event = Box::new(GameEventMessage::read(reader)?);\n");
        out.push_str(
            "                Ok(S2CMessage::OrderedGameEvent { object_id, sequence, event })\n",
        );
        out.push_str("            }\n");
    }

    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // Generate queue method
    out.push_str("    pub fn queue(&self) -> Option<MessageQueue> {\n");
    out.push_str("        match self {\n");

    for t in message_types {
        if let Some(ref queue) = t.queue {
            let name = ProtocolIdentifier::new(&t.name);
            let queue_name = ProtocolIdentifier::new(queue);
            out.push_str(&format!(
                "            S2CMessage::{}(_) => Some(MessageQueue::{}),\n",
                name.no_underscores(),
                queue_name.pascal_case()
            ));
        }
    }

    out.push_str("            S2CMessage::OrderedGameEvent { event, .. } => event.queue(),\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate ACWritable implementation
    out.push_str("impl ACWritable for S2CMessage {\n");
    out.push_str("    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {\n");
    out.push_str("        match self {\n");

    for protocol_type in message_types {
        if !protocol_type.is_primitive {
            let name = ProtocolIdentifier::new(&protocol_type.name);
            out.push_str(&format!(
                "            S2CMessage::{}(msg) => {{\n",
                name.no_underscores()
            ));
            out.push_str(&format!(
                "                write_u32(writer, crate::enums::S2CMessage::{} as u32)?;\n",
                name.no_underscores()
            ));
            out.push_str("                write_item(writer, msg)?;\n");
            out.push_str("                Ok(())\n");
            out.push_str("            }\n");
        }
    }

    out.push_str("            S2CMessage::OrderedGameEvent { object_id, sequence, event } => {\n");
    out.push_str("                write_u32(writer, crate::enums::S2CMessage::OrderedGameEvent as u32)?;\n");
    out.push_str("                write_u32(writer, *object_id)?;\n");
    out.push_str("                write_u32(writer, *sequence)?;\n");
    out.push_str("                write_item(writer, event.as_ref())?;\n");
    out.push_str("                Ok(())\n");
    out.push_str("            }\n");

    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}

/// Generate a message enum (C2SMessage or S2CMessage) with variants that contain message data
fn generate_message_enum(
    enum_name: &str,
    message_types: &[ProtocolType],
    module_prefix: &str,
    opcode_enum: Option<&ProtocolEnum>,
) -> String {
    let mut out = String::new();

    // Generate enum
    out.push_str(&format!("/// {} enum with message data\n", enum_name));
    out.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
    out.push_str(&format!("pub enum {} {{\n", enum_name));

    // Box large types to avoid stack overflow (>= 512 bytes on stack)
    // These types are determined by running: FILTER_TYPES="..." cargo build
    let large_types = match enum_name {
        "GameEventMessage" => vec!["Login_PlayerDescription", "Item_SetAppraiseInfo"],
        "GameActionMessage" => vec!["Character_CharacterOptionsEvent"],
        _ => vec![],
    };

    for protocol_type in message_types {
        if !protocol_type.is_primitive {
            let name = ProtocolIdentifier::new(&protocol_type.name);

            // Add variant with message data, boxing large variants
            if large_types.contains(&name.original()) {
                out.push_str(&format!(
                    "    {}(Box<{}::{}>),\n",
                    name.no_underscores(),
                    module_prefix,
                    name.no_underscores()
                ));
            } else {
                out.push_str(&format!(
                    "    {}({}::{}),\n",
                    name.no_underscores(),
                    module_prefix,
                    name.no_underscores()
                ));
            }
        }
    }

    out.push_str("}\n\n");

    // Generate read implementation using the opcode enum
    out.push_str(&format!("impl {} {{\n", enum_name));
    out.push_str("    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n");

    // Determine which enum to use based on the enum_name
    let enum_type_name = if enum_name == "GameActionMessage" {
        "GameAction"
    } else if enum_name == "GameEventMessage" {
        "GameEvent"
    } else {
        enum_name
    };

    out.push_str("        let opcode = read_u32(reader)?;\n");
    out.push_str(&format!(
        "        let opcode_enum = crate::enums::{}::try_from(opcode)\n",
        enum_type_name
    ));
    out.push_str(&format!(
        "            .map_err(|_| format!(\"Unknown {} opcode: 0x{{:04X}}\", opcode))?;\n\n",
        enum_name
    ));
    out.push_str("        match opcode_enum {\n");

    // If we have the opcode enum, use it to generate match arms
    if let Some(protocol_enum) = opcode_enum {
        for protocol_type in message_types {
            if !protocol_type.is_primitive {
                let name = ProtocolIdentifier::new(&protocol_type.name);

                // Find the corresponding enum value
                if let Some(_enum_value) = protocol_enum
                    .values
                    .iter()
                    .find(|v| v.name == name.original())
                {
                    // Check if this variant should be boxed
                    if large_types.contains(&name.original()) {
                        out.push_str(&format!(
                            "            crate::enums::{}::{} => Ok({}::{}(Box::new({}::{}::read(reader)?))),\n",
                            enum_type_name,
                            name.no_underscores(),
                            enum_name,
                            name.no_underscores(),
                            module_prefix,
                            name.no_underscores()
                        ));
                    } else {
                        out.push_str(&format!(
                            "            crate::enums::{}::{} => Ok({}::{}({}::{}::read(reader)?)),\n",
                            enum_type_name,
                            name.no_underscores(),
                            enum_name,
                            name.no_underscores(),
                            module_prefix,
                            name.no_underscores()
                        ));
                    }
                }
            }
        }
    }

    out.push_str("        }\n");
    out.push_str("    }\n\n");

    // Generate queue method
    out.push_str("    pub fn queue(&self) -> Option<MessageQueue> {\n");
    out.push_str("        match self {\n");

    for protocol_type in message_types {
        if !protocol_type.is_primitive
            && let Some(ref queue) = protocol_type.queue
        {
            let name = ProtocolIdentifier::new(&protocol_type.name);
            let queue_name = ProtocolIdentifier::new(queue);
            out.push_str(&format!(
                "            {}::{}(_) => Some(MessageQueue::{}),\n",
                enum_name,
                name.no_underscores(),
                queue_name.pascal_case()
            ));
        }
    }

    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Generate ACWritable implementation for GameActionMessage and GameEventMessage
    let enum_type_name = if enum_name == "GameActionMessage" {
        "GameAction"
    } else if enum_name == "GameEventMessage" {
        "GameEvent"
    } else {
        enum_name
    };

    out.push_str(&format!("impl ACWritable for {} {{\n", enum_name));
    out.push_str("    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {\n");
    out.push_str("        match self {\n");

    for protocol_type in message_types {
        if !protocol_type.is_primitive {
            let name = ProtocolIdentifier::new(&protocol_type.name);

            // Check if this variant should be boxed
            let large_types = match enum_name {
                "GameEventMessage" => vec!["Login_PlayerDescription", "Item_SetAppraiseInfo"],
                "GameActionMessage" => vec!["Character_CharacterOptionsEvent"],
                _ => vec![],
            };

            let is_boxed = large_types.contains(&name.original());

            out.push_str(&format!(
                "            {}::{}(msg) => {{\n",
                enum_name,
                name.no_underscores()
            ));

            out.push_str(&format!(
                "                write_u32(writer, crate::enums::{}::{} as u32)?;\n",
                enum_type_name,
                name.no_underscores()
            ));

            if is_boxed {
                out.push_str("                write_item(writer, msg.as_ref())?;\n");
            } else {
                out.push_str("                write_item(writer, msg)?;\n");
            }

            out.push_str("                Ok(())\n");
            out.push_str("            }\n");
        }
    }

    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}
