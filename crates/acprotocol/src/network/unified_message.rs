use serde::Serialize;
use std::io::{self, Cursor};

use crate::readers::ACReader;
use crate::unified::{Direction, MessageKind};

/// A parsed message using the unified message types
#[derive(Debug, Serialize)]
pub struct UnifiedMessage {
    /// Unique message ID
    pub id: u32,
    /// Position in the message stream
    pub sequence: u32,
    /// The parsed message
    #[serde(flatten)]
    pub message: MessageKind,
}

impl UnifiedMessage {
    /// Parse a message from assembled fragment data
    pub fn from_fragment(data: Vec<u8>, sequence: u32, id: u32) -> io::Result<Self> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message data too short to contain opcode",
            ));
        }

        // Determine direction based on opcode
        let opcode = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let direction = Self::determine_direction(opcode);

        // Parse the message
        let mut cursor = Cursor::new(&data[..]);
        let reader: &mut dyn ACReader = &mut cursor;

        let message = MessageKind::read(reader, direction)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(Self {
            id,
            sequence,
            message,
        })
    }

    /// Determine message direction based on opcode
    fn determine_direction(opcode: u32) -> Direction {
        use crate::enums::{C2SMessage, S2CMessage};

        if C2SMessage::try_from(opcode).is_ok() {
            Direction::ClientToServer
        } else if S2CMessage::try_from(opcode).is_ok() {
            Direction::ServerToClient
        } else {
            // Default to server to client for unknown opcodes
            Direction::ServerToClient
        }
    }
}
