use super::reader::BinaryReader;
use super::message_parser::parse_message_to_json;
use std::io;
use serde::Serialize;

/// A parsed message extracted from assembled fragments
#[derive(Debug, Clone, Serialize)]
pub struct ParsedMessage {
    /// Unique message ID
    pub id: u32,
    /// Opcode that identifies the message type (as hex string)
    #[serde(serialize_with = "serialize_opcode_hex")]
    pub opcode: u32,
    /// Parsed message data as JSON, or raw hex if parsing fails
    #[serde(serialize_with = "serialize_parsed_data")]
    pub data: Vec<u8>,
    /// Position in the message stream
    pub sequence: u32,
}

fn serialize_opcode_hex<S>(opcode: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex_string = format!("0x{:04X}", opcode);
    serializer.serialize_str(&hex_string)
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
        Err(_) => {
            // Fallback to hex string
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

        Ok(Self {
            id,
            opcode,
            data,
            sequence,
        })
    }

    /// Get a binary reader for reading message fields
    pub fn reader(&self) -> BinaryReader {
        BinaryReader::new(&self.data)
    }

    /// Get the opcode as hex string
    pub fn opcode_hex(&self) -> String {
        format!("0x{:04X}", self.opcode)
    }
}
