use super::message_parser::parse_message_to_json;
use super::reader::BinaryReader;
use serde::Serialize;
use std::io;

/// A parsed message extracted from assembled fragments
#[derive(Debug, Clone, Serialize)]
pub struct ParsedMessage {
    /// Unique message ID
    pub id: u32,
    /// Opcode that identifies the message type (as number)
    pub opcode: u32,
    /// Human-readable message type name
    pub message_type: String,
    /// Message direction (Send/Recv)
    pub direction: String,
    /// Parsed message data as JSON, or raw hex if parsing fails
    #[serde(serialize_with = "serialize_parsed_data")]
    pub data: Vec<u8>,
    /// Position in the message stream
    pub sequence: u32,
}

fn serialize_parsed_data<S>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Extract opcode from the data
    if data.len() < 4 {
        return serializer.serialize_str("invalid");
    }

    let opcode = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // Try to parse the data as a structured message
    match parse_message_to_json(opcode, data) {
        Ok(json_value) => json_value.serialize(serializer),
        Err(e) => {
            // Debug: log the error (only if it's unusual)
            let error_msg = format!("Failed to parse opcode 0x{:04x}: {}", opcode, e);
            if !error_msg.contains("No discriminant in enum") && !error_msg.contains("invalid utf-8") {
                eprintln!("{}", error_msg);
            }

            // Fallback to hex string if parsing fails
            let hex_string: String = data.iter().map(|b| format!("{b:02x}")).collect();
            serializer.serialize_str(&hex_string)
        }
    }
}

impl ParsedMessage {
    /// Parse a message from assembled fragment data
    pub fn from_fragment(data: Vec<u8>, sequence: u32, id: u32) -> io::Result<Self> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message data too short to contain opcode",
            ));
        }

        let opcode = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        // Create a temporary message to get type name and direction
        let message = Self {
            id,
            opcode,
            message_type: String::new(),
            direction: String::new(),
            data,
            sequence,
        };

        let message_type = message.message_type_name();
        let direction = message.direction().to_string();

        Ok(Self {
            id: message.id,
            opcode: message.opcode,
            message_type,
            direction,
            data: message.data,
            sequence: message.sequence,
        })
    }

    /// Get a binary reader for reading message fields
    pub fn reader(&self) -> BinaryReader<'_> {
        BinaryReader::new(&self.data)
    }

    /// Get the opcode as hex string
    pub fn opcode_hex(&self) -> String {
        format!("0x{:04X}", self.opcode)
    }

    /// Get the human-readable message type name
    pub fn message_type_name(&self) -> String {
        use crate::generated::enums::{C2SMessage, S2CMessage, GameEvent, GameAction};

        // Try C2S
        if let Ok(msg_type) = C2SMessage::try_from(self.opcode) {
            // For ordered messages, get the inner type from the payload
            if msg_type == C2SMessage::OrderedGameAction && self.data.len() >= 8 {
                // Read sequence (4 bytes) and action type (4 bytes)
                let action_type_val = u32::from_le_bytes([self.data[4], self.data[5], self.data[6], self.data[7]]);
                if let Ok(game_action) = GameAction::try_from(action_type_val) {
                    return format!("{:?}", game_action);
                }
                // Fallback to the outer type if we can't parse the inner type
                return "OrderedGameAction".to_string();
            }
            return format!("{:?}", msg_type);
        }

        // Try S2C
        if let Ok(msg_type) = S2CMessage::try_from(self.opcode) {
            // For ordered messages, get the inner type from the payload
            if msg_type == S2CMessage::OrderedGameEvent && self.data.len() >= 12 {
                // Read object ID (4 bytes), sequence (4 bytes), and event type (4 bytes)
                let event_type_val = u32::from_le_bytes([self.data[8], self.data[9], self.data[10], self.data[11]]);
                if let Ok(game_event) = GameEvent::try_from(event_type_val) {
                    return format!("{:?}", game_event);
                }
                // Fallback to the outer type if we can't parse the inner type
                return "OrderedGameEvent".to_string();
            }
            return format!("{:?}", msg_type);
        }

        "Unknown".to_string()
    }

    /// Get the message direction (Send/Recv)
    pub fn direction(&self) -> &'static str {
        use crate::generated::enums::{C2SMessage, S2CMessage};

        if C2SMessage::try_from(self.opcode).is_ok() {
            "Send"
        } else if S2CMessage::try_from(self.opcode).is_ok() {
            "Recv"
        } else {
            "Unknown"
        }
    }
}
