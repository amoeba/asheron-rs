use serde_json::json;
use std::io::Cursor;

use crate::generated::enums::{C2SMessage, S2CMessage};
use crate::readers::ACDataType;

/// Try to parse message data based on opcode and return as JSON
pub fn parse_message_to_json(
    opcode: u32,
    data: &[u8],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Skip opcode (first 4 bytes)
    if data.len() < 4 {
        return Err("Data too short".into());
    }

    let payload = &data[4..];
    let mut cursor = Cursor::new(payload);

    // Try C2S messages first
    if let Ok(msg_type) = C2SMessage::try_from(opcode) {
        return parse_c2s_message(msg_type, &mut cursor);
    }

    // Try S2C messages
    if let Ok(msg_type) = S2CMessage::try_from(opcode) {
        return parse_s2c_message(msg_type, &mut cursor);
    }

    // Unknown opcode, return as hex
    let hex_string: String = data.iter().map(|b| format!("{b:02x}")).collect();
    Ok(json!({ "hex": hex_string }))
}

fn parse_c2s_message(
    msg_type: C2SMessage,
    cursor: &mut Cursor<&[u8]>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Try to parse each message type - most will return a generic fallback
    let json = match msg_type {
        C2SMessage::CharacterSendCharGenResult => {
            use crate::messages::c2s::CharacterSendCharGenResult;
            serde_json::to_value(CharacterSendCharGenResult::read(cursor)?)?
        }
        C2SMessage::LoginSendEnterWorld => {
            use crate::messages::c2s::LoginSendEnterWorld;
            serde_json::to_value(LoginSendEnterWorld::read(cursor)?)?
        }
        C2SMessage::LoginSendEnterWorldRequest => {
            use crate::messages::c2s::LoginSendEnterWorldRequest;
            serde_json::to_value(LoginSendEnterWorldRequest::read(cursor)?)?
        }
        C2SMessage::CharacterCharacterDelete => {
            use crate::messages::c2s::CharacterCharacterDelete;
            serde_json::to_value(CharacterCharacterDelete::read(cursor)?)?
        }
        C2SMessage::LoginLogOffCharacter => {
            use crate::messages::c2s::LoginLogOffCharacter;
            serde_json::to_value(LoginLogOffCharacter::read(cursor)?)?
        }
        C2SMessage::CommunicationTurbineChat => {
            use crate::messages::c2s::CommunicationTurbineChat;
            serde_json::to_value(CommunicationTurbineChat::read(cursor)?)?
        }
        _ => {
            // Fallback to hex for unimplemented message types
            return Ok(json!({"type": "C2S", "opcode": format!("0x{:X}", msg_type as u32)}));
        }
    };

    Ok(json)
}

fn parse_s2c_message(
    msg_type: S2CMessage,
    cursor: &mut Cursor<&[u8]>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json = match msg_type {
        S2CMessage::QualitiesUpdateInt => {
            use crate::messages::s2c::QualitiesUpdateInt;
            serde_json::to_value(QualitiesUpdateInt::read(cursor)?)?
        }
        S2CMessage::QualitiesUpdateBool => {
            use crate::messages::s2c::QualitiesUpdateBool;
            serde_json::to_value(QualitiesUpdateBool::read(cursor)?)?
        }
        S2CMessage::QualitiesUpdateString => {
            use crate::messages::s2c::QualitiesUpdateString;
            serde_json::to_value(QualitiesUpdateString::read(cursor)?)?
        }
        S2CMessage::CharacterServerSaysAttemptFailed => {
            use crate::messages::s2c::CharacterServerSaysAttemptFailed;
            serde_json::to_value(CharacterServerSaysAttemptFailed::read(cursor)?)?
        }
        S2CMessage::ItemCreateObject => {
            use crate::messages::s2c::ItemCreateObject;
            serde_json::to_value(ItemCreateObject::read(cursor)?)?
        }
        S2CMessage::ItemDeleteObject => {
            use crate::messages::s2c::ItemDeleteObject;
            serde_json::to_value(ItemDeleteObject::read(cursor)?)?
        }
        S2CMessage::MovementPositionEvent => {
            use crate::messages::s2c::MovementPositionEvent;
            serde_json::to_value(MovementPositionEvent::read(cursor)?)?
        }
        S2CMessage::ItemServerSaysRemove => {
            use crate::messages::s2c::ItemServerSaysRemove;
            serde_json::to_value(ItemServerSaysRemove::read(cursor)?)?
        }
        S2CMessage::InventoryPickupEvent => {
            use crate::messages::s2c::InventoryPickupEvent;
            serde_json::to_value(InventoryPickupEvent::read(cursor)?)?
        }
        S2CMessage::LoginCreatePlayer => {
            use crate::messages::s2c::LoginCreatePlayer;
            serde_json::to_value(LoginCreatePlayer::read(cursor)?)?
        }
        S2CMessage::ItemParentEvent => {
            use crate::messages::s2c::ItemParentEvent;
            serde_json::to_value(ItemParentEvent::read(cursor)?)?
        }
        S2CMessage::ItemUpdateObject => {
            use crate::messages::s2c::ItemUpdateObject;
            serde_json::to_value(ItemUpdateObject::read(cursor)?)?
        }
        _ => {
            // Fallback to hex for unimplemented message types
            return Ok(json!({"type": "S2C", "opcode": format!("0x{:X}", msg_type as u32)}));
        }
    };

    Ok(json)
}
