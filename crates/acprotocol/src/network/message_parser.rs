use serde_json::json;
use std::io::Cursor;

use crate::generated::enums::{C2SMessage, S2CMessage};
use crate::readers::{ACReader, ACDataType};

/// Try to parse message data based on opcode and return as JSON
pub fn parse_message_to_json(
    opcode: u32,
    data: &[u8],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Try C2S messages first
    if let Ok(msg_type) = C2SMessage::try_from(opcode) {
        return parse_c2s_message(msg_type, data);
    }

    // Try S2C messages
    if let Ok(msg_type) = S2CMessage::try_from(opcode) {
        return parse_s2c_message(msg_type, data);
    }

    // Unknown opcode, return as hex
    let hex_string: String = data.iter().map(|b| format!("{b:02x}")).collect();
    Ok(json!({ "hex": hex_string }))
}

fn parse_c2s_message(
    msg_type: C2SMessage,
    data: &[u8],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Skip opcode (first 4 bytes)
    if data.len() < 4 {
        return Err("Data too short".into());
    }

    let payload = &data[4..];
    let mut cursor = Cursor::new(payload);

    let json = match msg_type {
        C2SMessage::CharacterSendCharGenResult => {
            use crate::generated::messages::c2s::character_send_char_gen_result::CharacterSendCharGenResult;
            serde_json::to_value(CharacterSendCharGenResult::read(&mut cursor)?)?
        }
        C2SMessage::LoginSendEnterWorld => {
            use crate::generated::messages::c2s::login_send_enter_world::LoginSendEnterWorld;
            serde_json::to_value(LoginSendEnterWorld::read(&mut cursor)?)?
        }
        C2SMessage::LoginSendEnterWorldRequest => {
            use crate::generated::messages::c2s::login_send_enter_world_request::LoginSendEnterWorldRequest;
            serde_json::to_value(LoginSendEnterWorldRequest::read(&mut cursor)?)?
        }
        C2SMessage::CharacterCharacterDelete => {
            use crate::generated::messages::c2s::character_character_delete::CharacterCharacterDelete;
            serde_json::to_value(CharacterCharacterDelete::read(&mut cursor)?)?
        }
        C2SMessage::LoginLogOffCharacter => {
            use crate::generated::messages::c2s::login_log_off_character::LoginLogOffCharacter;
            serde_json::to_value(LoginLogOffCharacter::read(&mut cursor)?)?
        }
        C2SMessage::CommunicationTurbineChat => {
            use crate::generated::messages::c2s::communication_turbine_chat::CommunicationTurbineChat;
            serde_json::to_value(CommunicationTurbineChat::read(&mut cursor)?)?
        }
        _ => {
            // Fallback to hex for unimplemented message types
            let hex_string: String = data.iter().map(|b| format!("{b:02x}")).collect();
            return Ok(json!({ "hex": hex_string }));
        }
    };

    Ok(json)
}

fn parse_s2c_message(
    msg_type: S2CMessage,
    data: &[u8],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Skip opcode (first 4 bytes)
    if data.len() < 4 {
        return Err("Data too short".into());
    }

    let payload = &data[4..];
    let mut cursor = Cursor::new(payload);

    let json = match msg_type {
        S2CMessage::QualitiesUpdateInt => {
            use crate::generated::messages::s2c::qualities_update_int::QualitiesUpdateInt;
            serde_json::to_value(QualitiesUpdateInt::read(&mut cursor)?)?
        }
        S2CMessage::QualitiesUpdateBool => {
            use crate::generated::messages::s2c::qualities_update_bool::QualitiesUpdateBool;
            serde_json::to_value(QualitiesUpdateBool::read(&mut cursor)?)?
        }
        S2CMessage::QualitiesUpdateString => {
            use crate::generated::messages::s2c::qualities_update_string::QualitiesUpdateString;
            serde_json::to_value(QualitiesUpdateString::read(&mut cursor)?)?
        }
        S2CMessage::CharacterServerSaysAttemptFailed => {
            use crate::generated::messages::s2c::character_server_says_attempt_failed::CharacterServerSaysAttemptFailed;
            serde_json::to_value(CharacterServerSaysAttemptFailed::read(&mut cursor)?)?
        }
        S2CMessage::ItemCreateObject => {
            use crate::generated::messages::s2c::item_create_object::ItemCreateObject;
            serde_json::to_value(ItemCreateObject::read(&mut cursor)?)?
        }
        S2CMessage::ItemDeleteObject => {
            use crate::generated::messages::s2c::item_delete_object::ItemDeleteObject;
            serde_json::to_value(ItemDeleteObject::read(&mut cursor)?)?
        }
        S2CMessage::MovementPositionEvent => {
            use crate::generated::messages::s2c::movement_position_event::MovementPositionEvent;
            serde_json::to_value(MovementPositionEvent::read(&mut cursor)?)?
        }
        S2CMessage::ItemServerSaysRemove => {
            use crate::generated::messages::s2c::item_server_says_remove::ItemServerSaysRemove;
            serde_json::to_value(ItemServerSaysRemove::read(&mut cursor)?)?
        }
        S2CMessage::InventoryPickupEvent => {
            use crate::generated::messages::s2c::inventory_pickup_event::InventoryPickupEvent;
            serde_json::to_value(InventoryPickupEvent::read(&mut cursor)?)?
        }
        S2CMessage::LoginCreatePlayer => {
            use crate::generated::messages::s2c::login_create_player::LoginCreatePlayer;
            serde_json::to_value(LoginCreatePlayer::read(&mut cursor)?)?
        }
        S2CMessage::ItemParentEvent => {
            use crate::generated::messages::s2c::item_parent_event::ItemParentEvent;
            serde_json::to_value(ItemParentEvent::read(&mut cursor)?)?
        }
        S2CMessage::ItemUpdateObject => {
            use crate::generated::messages::s2c::item_update_object::ItemUpdateObject;
            serde_json::to_value(ItemUpdateObject::read(&mut cursor)?)?
        }
        _ => {
            // Fallback to hex for unimplemented message types
            let hex_string: String = data.iter().map(|b| format!("{b:02x}")).collect();
            return Ok(json!({ "hex": hex_string }));
        }
    };

    Ok(json)
}
