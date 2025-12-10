// Generated message parser with 14 C2S and 91 S2C types
use serde_json::json;
use std::io::{Cursor, Seek};
use crate::generated::enums::{C2SMessage, S2CMessage, GameEvent, GameAction};
use crate::readers::ACReader;

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
    // Special handling for OrderedGameAction
    if msg_type == C2SMessage::OrderedGameAction {
        return parse_game_action(cursor);
    }

    match msg_type {
        C2SMessage::LoginLogOffCharacter => {
            use crate::messages::c2s::LoginLogOffCharacter;
            serde_json::to_value(LoginLogOffCharacter::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::CharacterCharacterDelete => {
            use crate::messages::c2s::CharacterCharacterDelete;
            serde_json::to_value(CharacterCharacterDelete::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::CharacterSendCharGenResult => {
            use crate::messages::c2s::CharacterSendCharGenResult;
            serde_json::to_value(CharacterSendCharGenResult::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::LoginSendEnterWorld => {
            use crate::messages::c2s::LoginSendEnterWorld;
            serde_json::to_value(LoginSendEnterWorld::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::ObjectSendForceObjdesc => {
            use crate::messages::c2s::ObjectSendForceObjdesc;
            serde_json::to_value(ObjectSendForceObjdesc::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::LoginSendEnterWorldRequest => {
            use crate::messages::c2s::LoginSendEnterWorldRequest;
            serde_json::to_value(LoginSendEnterWorldRequest::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::AdminSendAdminGetServerVersion => {
            use crate::messages::c2s::AdminSendAdminGetServerVersion;
            serde_json::to_value(AdminSendAdminGetServerVersion::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::SocialSendFriendsCommand => {
            use crate::messages::c2s::SocialSendFriendsCommand;
            serde_json::to_value(SocialSendFriendsCommand::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::AdminSendAdminRestoreCharacter => {
            use crate::messages::c2s::AdminSendAdminRestoreCharacter;
            serde_json::to_value(AdminSendAdminRestoreCharacter::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::CommunicationTurbineChat => {
            use crate::messages::c2s::CommunicationTurbineChat;
            serde_json::to_value(CommunicationTurbineChat::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::DDDRequestDataMessage => {
            use crate::messages::c2s::DDDRequestDataMessage;
            serde_json::to_value(DDDRequestDataMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::DDDInterrogationResponseMessage => {
            use crate::messages::c2s::DDDInterrogationResponseMessage;
            serde_json::to_value(DDDInterrogationResponseMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::DDDEndDDDMessage => {
            use crate::messages::c2s::DDDEndDDDMessage;
            serde_json::to_value(DDDEndDDDMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        C2SMessage::DDDOnEndDDD => {
            use crate::messages::c2s::DDDOnEndDDD;
            serde_json::to_value(DDDOnEndDDD::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        _ => {
            // Fallback to basic info for unimplemented message types
            Ok(json!({"opcode": msg_type as u32, "type": "C2S"}))
        }
    }
}

fn parse_s2c_message(
    msg_type: S2CMessage,
    cursor: &mut Cursor<&[u8]>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Special handling for OrderedGameEvent
    if msg_type == S2CMessage::OrderedGameEvent {
        return parse_game_event(cursor);
    }

    match msg_type {
        S2CMessage::ItemServerSaysRemove => {
            use crate::messages::s2c::ItemServerSaysRemove;
            serde_json::to_value(ItemServerSaysRemove::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CharacterServerSaysAttemptFailed => {
            use crate::messages::s2c::CharacterServerSaysAttemptFailed;
            serde_json::to_value(CharacterServerSaysAttemptFailed::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemUpdateStackSize => {
            use crate::messages::s2c::ItemUpdateStackSize;
            serde_json::to_value(ItemUpdateStackSize::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CombatHandlePlayerDeathEvent => {
            use crate::messages::s2c::CombatHandlePlayerDeathEvent;
            serde_json::to_value(CombatHandlePlayerDeathEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveIntEvent => {
            use crate::messages::s2c::QualitiesPrivateRemoveIntEvent;
            serde_json::to_value(QualitiesPrivateRemoveIntEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveIntEvent => {
            use crate::messages::s2c::QualitiesRemoveIntEvent;
            serde_json::to_value(QualitiesRemoveIntEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveBoolEvent => {
            use crate::messages::s2c::QualitiesPrivateRemoveBoolEvent;
            serde_json::to_value(QualitiesPrivateRemoveBoolEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveBoolEvent => {
            use crate::messages::s2c::QualitiesRemoveBoolEvent;
            serde_json::to_value(QualitiesRemoveBoolEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveFloatEvent => {
            use crate::messages::s2c::QualitiesPrivateRemoveFloatEvent;
            serde_json::to_value(QualitiesPrivateRemoveFloatEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveFloatEvent => {
            use crate::messages::s2c::QualitiesRemoveFloatEvent;
            serde_json::to_value(QualitiesRemoveFloatEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveStringEvent => {
            use crate::messages::s2c::QualitiesPrivateRemoveStringEvent;
            serde_json::to_value(QualitiesPrivateRemoveStringEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveStringEvent => {
            use crate::messages::s2c::QualitiesRemoveStringEvent;
            serde_json::to_value(QualitiesRemoveStringEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveDataIdEvent => {
            use crate::messages::s2c::QualitiesPrivateRemoveDataIdEvent;
            serde_json::to_value(QualitiesPrivateRemoveDataIdEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveDataIdEvent => {
            use crate::messages::s2c::QualitiesRemoveDataIdEvent;
            serde_json::to_value(QualitiesRemoveDataIdEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveInstanceIdEvent => {
            use crate::messages::s2c::QualitiesPrivateRemoveInstanceIdEvent;
            serde_json::to_value(QualitiesPrivateRemoveInstanceIdEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveInstanceIdEvent => {
            use crate::messages::s2c::QualitiesRemoveInstanceIdEvent;
            serde_json::to_value(QualitiesRemoveInstanceIdEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemovePositionEvent => {
            use crate::messages::s2c::QualitiesPrivateRemovePositionEvent;
            serde_json::to_value(QualitiesPrivateRemovePositionEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemovePositionEvent => {
            use crate::messages::s2c::QualitiesRemovePositionEvent;
            serde_json::to_value(QualitiesRemovePositionEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateRemoveInt64Event => {
            use crate::messages::s2c::QualitiesPrivateRemoveInt64Event;
            serde_json::to_value(QualitiesPrivateRemoveInt64Event::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesRemoveInt64Event => {
            use crate::messages::s2c::QualitiesRemoveInt64Event;
            serde_json::to_value(QualitiesRemoveInt64Event::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateInt => {
            use crate::messages::s2c::QualitiesPrivateUpdateInt;
            serde_json::to_value(QualitiesPrivateUpdateInt::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateInt => {
            use crate::messages::s2c::QualitiesUpdateInt;
            serde_json::to_value(QualitiesUpdateInt::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateInt64 => {
            use crate::messages::s2c::QualitiesPrivateUpdateInt64;
            serde_json::to_value(QualitiesPrivateUpdateInt64::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateInt64 => {
            use crate::messages::s2c::QualitiesUpdateInt64;
            serde_json::to_value(QualitiesUpdateInt64::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateBool => {
            use crate::messages::s2c::QualitiesPrivateUpdateBool;
            serde_json::to_value(QualitiesPrivateUpdateBool::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateBool => {
            use crate::messages::s2c::QualitiesUpdateBool;
            serde_json::to_value(QualitiesUpdateBool::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateFloat => {
            use crate::messages::s2c::QualitiesPrivateUpdateFloat;
            serde_json::to_value(QualitiesPrivateUpdateFloat::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateFloat => {
            use crate::messages::s2c::QualitiesUpdateFloat;
            serde_json::to_value(QualitiesUpdateFloat::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateString => {
            use crate::messages::s2c::QualitiesPrivateUpdateString;
            serde_json::to_value(QualitiesPrivateUpdateString::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateString => {
            use crate::messages::s2c::QualitiesUpdateString;
            serde_json::to_value(QualitiesUpdateString::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateDataId => {
            use crate::messages::s2c::QualitiesPrivateUpdateDataId;
            serde_json::to_value(QualitiesPrivateUpdateDataId::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateDataId => {
            use crate::messages::s2c::QualitiesUpdateDataId;
            serde_json::to_value(QualitiesUpdateDataId::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateInstanceId => {
            use crate::messages::s2c::QualitiesPrivateUpdateInstanceId;
            serde_json::to_value(QualitiesPrivateUpdateInstanceId::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateInstanceId => {
            use crate::messages::s2c::QualitiesUpdateInstanceId;
            serde_json::to_value(QualitiesUpdateInstanceId::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdatePosition => {
            use crate::messages::s2c::QualitiesPrivateUpdatePosition;
            serde_json::to_value(QualitiesPrivateUpdatePosition::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdatePosition => {
            use crate::messages::s2c::QualitiesUpdatePosition;
            serde_json::to_value(QualitiesUpdatePosition::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateSkill => {
            use crate::messages::s2c::QualitiesPrivateUpdateSkill;
            serde_json::to_value(QualitiesPrivateUpdateSkill::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateSkill => {
            use crate::messages::s2c::QualitiesUpdateSkill;
            serde_json::to_value(QualitiesUpdateSkill::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateSkillLevel => {
            use crate::messages::s2c::QualitiesPrivateUpdateSkillLevel;
            serde_json::to_value(QualitiesPrivateUpdateSkillLevel::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateSkillLevel => {
            use crate::messages::s2c::QualitiesUpdateSkillLevel;
            serde_json::to_value(QualitiesUpdateSkillLevel::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateSkillAC => {
            use crate::messages::s2c::QualitiesPrivateUpdateSkillAC;
            serde_json::to_value(QualitiesPrivateUpdateSkillAC::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateSkillAC => {
            use crate::messages::s2c::QualitiesUpdateSkillAC;
            serde_json::to_value(QualitiesUpdateSkillAC::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateAttribute => {
            use crate::messages::s2c::QualitiesPrivateUpdateAttribute;
            serde_json::to_value(QualitiesPrivateUpdateAttribute::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateAttribute => {
            use crate::messages::s2c::QualitiesUpdateAttribute;
            serde_json::to_value(QualitiesUpdateAttribute::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateAttributeLevel => {
            use crate::messages::s2c::QualitiesPrivateUpdateAttributeLevel;
            serde_json::to_value(QualitiesPrivateUpdateAttributeLevel::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateAttributeLevel => {
            use crate::messages::s2c::QualitiesUpdateAttributeLevel;
            serde_json::to_value(QualitiesUpdateAttributeLevel::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateAttribute2nd => {
            use crate::messages::s2c::QualitiesPrivateUpdateAttribute2nd;
            serde_json::to_value(QualitiesPrivateUpdateAttribute2nd::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateAttribute2nd => {
            use crate::messages::s2c::QualitiesUpdateAttribute2nd;
            serde_json::to_value(QualitiesUpdateAttribute2nd::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesPrivateUpdateAttribute2ndLevel => {
            use crate::messages::s2c::QualitiesPrivateUpdateAttribute2ndLevel;
            serde_json::to_value(QualitiesPrivateUpdateAttribute2ndLevel::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::QualitiesUpdateAttribute2ndLevel => {
            use crate::messages::s2c::QualitiesUpdateAttribute2ndLevel;
            serde_json::to_value(QualitiesUpdateAttribute2ndLevel::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CommunicationHearEmote => {
            use crate::messages::s2c::CommunicationHearEmote;
            serde_json::to_value(CommunicationHearEmote::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CommunicationHearSoulEmote => {
            use crate::messages::s2c::CommunicationHearSoulEmote;
            serde_json::to_value(CommunicationHearSoulEmote::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CommunicationHearSpeech => {
            use crate::messages::s2c::CommunicationHearSpeech;
            serde_json::to_value(CommunicationHearSpeech::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CommunicationHearRangedSpeech => {
            use crate::messages::s2c::CommunicationHearRangedSpeech;
            serde_json::to_value(CommunicationHearRangedSpeech::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::AdminEnvirons => {
            use crate::messages::s2c::AdminEnvirons;
            serde_json::to_value(AdminEnvirons::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::MovementPositionAndMovementEvent => {
            use crate::messages::s2c::MovementPositionAndMovementEvent;
            serde_json::to_value(MovementPositionAndMovementEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemObjDescEvent => {
            use crate::messages::s2c::ItemObjDescEvent;
            serde_json::to_value(ItemObjDescEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CharacterSetPlayerVisualDesc => {
            use crate::messages::s2c::CharacterSetPlayerVisualDesc;
            serde_json::to_value(CharacterSetPlayerVisualDesc::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CharacterCharGenVerificationResponse => {
            use crate::messages::s2c::CharacterCharGenVerificationResponse;
            serde_json::to_value(CharacterCharGenVerificationResponse::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginAwaitingSubscriptionExpiration => {
            use crate::messages::s2c::LoginAwaitingSubscriptionExpiration;
            serde_json::to_value(LoginAwaitingSubscriptionExpiration::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginLogOffCharacter => {
            use crate::messages::s2c::LoginLogOffCharacter;
            serde_json::to_value(LoginLogOffCharacter::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CharacterCharacterDelete => {
            use crate::messages::s2c::CharacterCharacterDelete;
            serde_json::to_value(CharacterCharacterDelete::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginLoginCharacterSet => {
            use crate::messages::s2c::LoginLoginCharacterSet;
            serde_json::to_value(LoginLoginCharacterSet::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CharacterCharacterError => {
            use crate::messages::s2c::CharacterCharacterError;
            serde_json::to_value(CharacterCharacterError::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemCreateObject => {
            use crate::messages::s2c::ItemCreateObject;
            serde_json::to_value(ItemCreateObject::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginCreatePlayer => {
            use crate::messages::s2c::LoginCreatePlayer;
            serde_json::to_value(LoginCreatePlayer::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemDeleteObject => {
            use crate::messages::s2c::ItemDeleteObject;
            serde_json::to_value(ItemDeleteObject::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::MovementPositionEvent => {
            use crate::messages::s2c::MovementPositionEvent;
            serde_json::to_value(MovementPositionEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemParentEvent => {
            use crate::messages::s2c::ItemParentEvent;
            serde_json::to_value(ItemParentEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::InventoryPickupEvent => {
            use crate::messages::s2c::InventoryPickupEvent;
            serde_json::to_value(InventoryPickupEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemSetState => {
            use crate::messages::s2c::ItemSetState;
            serde_json::to_value(ItemSetState::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::MovementSetObjectMovement => {
            use crate::messages::s2c::MovementSetObjectMovement;
            serde_json::to_value(MovementSetObjectMovement::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::MovementVectorUpdate => {
            use crate::messages::s2c::MovementVectorUpdate;
            serde_json::to_value(MovementVectorUpdate::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::EffectsSoundEvent => {
            use crate::messages::s2c::EffectsSoundEvent;
            serde_json::to_value(EffectsSoundEvent::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::EffectsPlayerTeleport => {
            use crate::messages::s2c::EffectsPlayerTeleport;
            serde_json::to_value(EffectsPlayerTeleport::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::EffectsPlayScriptId => {
            use crate::messages::s2c::EffectsPlayScriptId;
            serde_json::to_value(EffectsPlayScriptId::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::EffectsPlayScriptType => {
            use crate::messages::s2c::EffectsPlayScriptType;
            serde_json::to_value(EffectsPlayScriptType::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginAccountBanned => {
            use crate::messages::s2c::LoginAccountBanned;
            serde_json::to_value(LoginAccountBanned::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::AdminReceiveAccountData => {
            use crate::messages::s2c::AdminReceiveAccountData;
            serde_json::to_value(AdminReceiveAccountData::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::AdminReceivePlayerData => {
            use crate::messages::s2c::AdminReceivePlayerData;
            serde_json::to_value(AdminReceivePlayerData::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::ItemUpdateObject => {
            use crate::messages::s2c::ItemUpdateObject;
            serde_json::to_value(ItemUpdateObject::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginAccountBooted => {
            use crate::messages::s2c::LoginAccountBooted;
            serde_json::to_value(LoginAccountBooted::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CommunicationTurbineChat => {
            use crate::messages::s2c::CommunicationTurbineChat;
            serde_json::to_value(CommunicationTurbineChat::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginEnterGameServerReady => {
            use crate::messages::s2c::LoginEnterGameServerReady;
            serde_json::to_value(LoginEnterGameServerReady::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::CommunicationTextboxString => {
            use crate::messages::s2c::CommunicationTextboxString;
            serde_json::to_value(CommunicationTextboxString::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::LoginWorldInfo => {
            use crate::messages::s2c::LoginWorldInfo;
            serde_json::to_value(LoginWorldInfo::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::DDDDataMessage => {
            use crate::messages::s2c::DDDDataMessage;
            serde_json::to_value(DDDDataMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::DDDErrorMessage => {
            use crate::messages::s2c::DDDErrorMessage;
            serde_json::to_value(DDDErrorMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::DDDBeginDDDMessage => {
            use crate::messages::s2c::DDDBeginDDDMessage;
            serde_json::to_value(DDDBeginDDDMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::DDDInterrogationMessage => {
            use crate::messages::s2c::DDDInterrogationMessage;
            serde_json::to_value(DDDInterrogationMessage::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        S2CMessage::DDDOnEndDDD => {
            use crate::messages::s2c::DDDOnEndDDD;
            serde_json::to_value(DDDOnEndDDD::read(cursor)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        _ => {
            // Fallback to basic info for unimplemented message types
            Ok(json!({"opcode": msg_type as u32, "type": "S2C"}))
        }
    }
}

fn parse_game_event(
    cursor: &mut Cursor<&[u8]>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Store initial position before reading header
    let _initial_pos = cursor.position();
    
    // Read header fields to determine the event type (as per Chorizite pattern)
    let object_id = crate::readers::read_u32(&mut Box::new(cursor.clone()))?;
    let sequence = crate::readers::read_u32(&mut Box::new(cursor.clone()))?;
    let event_type_value = crate::readers::read_u32(&mut Box::new(cursor.clone()))?;
    
    let event_type = match GameEvent::try_from(event_type_value) {
        Ok(et) => et,
        Err(_) => return Ok(json!({
            "objectId": object_id,
            "sequence": sequence,
            "eventType": event_type_value,
            "error": "Unknown event type"
        })),
    };

    // Backtrack to allow the specific struct to read the full payload (as per Chorizite pattern)
    let pos = cursor.position();
    if pos >= 12 {
        cursor.seek(std::io::SeekFrom::Current(-12))?;  // Backtrack 3 u32s = 12 bytes
    } else {
        cursor.set_position(0);
    }
    
    // Create a reader from the current cursor position to parse the full payload
    let mut reader: Box<dyn ACReader> = Box::new(cursor.clone());
    
    // Parse using the specific event struct (which will read header + payload)
    let event_data: Result<serde_json::Value, Box<dyn std::error::Error>> = match event_type {
        GameEvent::AllegianceAllegianceUpdateAborted => {
            use crate::generated::gameevents::allegiance_allegiance_update_aborted::AllegianceAllegianceUpdateAborted;
            serde_json::to_value(AllegianceAllegianceUpdateAborted::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationPopUpString => {
            use crate::generated::gameevents::communication_pop_up_string::CommunicationPopUpString;
            serde_json::to_value(CommunicationPopUpString::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::LoginPlayerDescription => {
            use crate::generated::gameevents::login_player_description::LoginPlayerDescription;
            serde_json::to_value(LoginPlayerDescription::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AllegianceAllegianceUpdate => {
            use crate::generated::gameevents::allegiance_allegiance_update::AllegianceAllegianceUpdate;
            serde_json::to_value(AllegianceAllegianceUpdate::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::SocialFriendsUpdate => {
            use crate::generated::gameevents::social_friends_update::SocialFriendsUpdate;
            serde_json::to_value(SocialFriendsUpdate::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemServerSaysContainId => {
            use crate::generated::gameevents::item_server_says_contain_id::ItemServerSaysContainId;
            serde_json::to_value(ItemServerSaysContainId::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemWearItem => {
            use crate::generated::gameevents::item_wear_item::ItemWearItem;
            serde_json::to_value(ItemWearItem::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::SocialCharacterTitleTable => {
            use crate::generated::gameevents::social_character_title_table::SocialCharacterTitleTable;
            serde_json::to_value(SocialCharacterTitleTable::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::SocialAddOrSetCharacterTitle => {
            use crate::generated::gameevents::social_add_or_set_character_title::SocialAddOrSetCharacterTitle;
            serde_json::to_value(SocialAddOrSetCharacterTitle::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemStopViewingObjectContents => {
            use crate::generated::gameevents::item_stop_viewing_object_contents::ItemStopViewingObjectContents;
            serde_json::to_value(ItemStopViewingObjectContents::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::VendorVendorInfo => {
            use crate::generated::gameevents::vendor_vendor_info::VendorVendorInfo;
            serde_json::to_value(VendorVendorInfo::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CharacterStartBarber => {
            use crate::generated::gameevents::character_start_barber::CharacterStartBarber;
            serde_json::to_value(CharacterStartBarber::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipQuit => {
            use crate::generated::gameevents::fellowship_quit::FellowshipQuit;
            serde_json::to_value(FellowshipQuit::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipDismiss => {
            use crate::generated::gameevents::fellowship_dismiss::FellowshipDismiss;
            serde_json::to_value(FellowshipDismiss::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::WritingBookOpen => {
            use crate::generated::gameevents::writing_book_open::WritingBookOpen;
            serde_json::to_value(WritingBookOpen::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::WritingBookAddPageResponse => {
            use crate::generated::gameevents::writing_book_add_page_response::WritingBookAddPageResponse;
            serde_json::to_value(WritingBookAddPageResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::WritingBookDeletePageResponse => {
            use crate::generated::gameevents::writing_book_delete_page_response::WritingBookDeletePageResponse;
            serde_json::to_value(WritingBookDeletePageResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::WritingBookPageDataResponse => {
            use crate::generated::gameevents::writing_book_page_data_response::WritingBookPageDataResponse;
            serde_json::to_value(WritingBookPageDataResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemGetInscriptionResponse => {
            use crate::generated::gameevents::item_get_inscription_response::ItemGetInscriptionResponse;
            serde_json::to_value(ItemGetInscriptionResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemSetAppraiseInfo => {
            use crate::generated::gameevents::item_set_appraise_info::ItemSetAppraiseInfo;
            serde_json::to_value(ItemSetAppraiseInfo::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationChannelBroadcast => {
            use crate::generated::gameevents::communication_channel_broadcast::CommunicationChannelBroadcast;
            serde_json::to_value(CommunicationChannelBroadcast::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationChannelList => {
            use crate::generated::gameevents::communication_channel_list::CommunicationChannelList;
            serde_json::to_value(CommunicationChannelList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationChannelIndex => {
            use crate::generated::gameevents::communication_channel_index::CommunicationChannelIndex;
            serde_json::to_value(CommunicationChannelIndex::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemOnViewContents => {
            use crate::generated::gameevents::item_on_view_contents::ItemOnViewContents;
            serde_json::to_value(ItemOnViewContents::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemServerSaysMoveItem => {
            use crate::generated::gameevents::item_server_says_move_item::ItemServerSaysMoveItem;
            serde_json::to_value(ItemServerSaysMoveItem::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleAttackDoneEvent => {
            use crate::generated::gameevents::combat_handle_attack_done_event::CombatHandleAttackDoneEvent;
            serde_json::to_value(CombatHandleAttackDoneEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicRemoveSpell => {
            use crate::generated::gameevents::magic_remove_spell::MagicRemoveSpell;
            serde_json::to_value(MagicRemoveSpell::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleVictimNotificationEventSelf => {
            use crate::generated::gameevents::combat_handle_victim_notification_event_self::CombatHandleVictimNotificationEventSelf;
            serde_json::to_value(CombatHandleVictimNotificationEventSelf::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleVictimNotificationEventOther => {
            use crate::generated::gameevents::combat_handle_victim_notification_event_other::CombatHandleVictimNotificationEventOther;
            serde_json::to_value(CombatHandleVictimNotificationEventOther::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleAttackerNotificationEvent => {
            use crate::generated::gameevents::combat_handle_attacker_notification_event::CombatHandleAttackerNotificationEvent;
            serde_json::to_value(CombatHandleAttackerNotificationEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleDefenderNotificationEvent => {
            use crate::generated::gameevents::combat_handle_defender_notification_event::CombatHandleDefenderNotificationEvent;
            serde_json::to_value(CombatHandleDefenderNotificationEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleEvasionAttackerNotificationEvent => {
            use crate::generated::gameevents::combat_handle_evasion_attacker_notification_event::CombatHandleEvasionAttackerNotificationEvent;
            serde_json::to_value(CombatHandleEvasionAttackerNotificationEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleEvasionDefenderNotificationEvent => {
            use crate::generated::gameevents::combat_handle_evasion_defender_notification_event::CombatHandleEvasionDefenderNotificationEvent;
            serde_json::to_value(CombatHandleEvasionDefenderNotificationEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatHandleCommenceAttackEvent => {
            use crate::generated::gameevents::combat_handle_commence_attack_event::CombatHandleCommenceAttackEvent;
            serde_json::to_value(CombatHandleCommenceAttackEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CombatQueryHealthResponse => {
            use crate::generated::gameevents::combat_query_health_response::CombatQueryHealthResponse;
            serde_json::to_value(CombatQueryHealthResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CharacterQueryAgeResponse => {
            use crate::generated::gameevents::character_query_age_response::CharacterQueryAgeResponse;
            serde_json::to_value(CharacterQueryAgeResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemUseDone => {
            use crate::generated::gameevents::item_use_done::ItemUseDone;
            serde_json::to_value(ItemUseDone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AllegianceAllegianceUpdateDone => {
            use crate::generated::gameevents::allegiance_allegiance_update_done::AllegianceAllegianceUpdateDone;
            serde_json::to_value(AllegianceAllegianceUpdateDone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipFellowUpdateDone => {
            use crate::generated::gameevents::fellowship_fellow_update_done::FellowshipFellowUpdateDone;
            serde_json::to_value(FellowshipFellowUpdateDone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipFellowStatsDone => {
            use crate::generated::gameevents::fellowship_fellow_stats_done::FellowshipFellowStatsDone;
            serde_json::to_value(FellowshipFellowStatsDone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemAppraiseDone => {
            use crate::generated::gameevents::item_appraise_done::ItemAppraiseDone;
            serde_json::to_value(ItemAppraiseDone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CharacterReturnPing => {
            use crate::generated::gameevents::character_return_ping::CharacterReturnPing;
            serde_json::to_value(CharacterReturnPing::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationSetSquelchDB => {
            use crate::generated::gameevents::communication_set_squelch_db::CommunicationSetSquelchDB;
            serde_json::to_value(CommunicationSetSquelchDB::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeRegisterTrade => {
            use crate::generated::gameevents::trade_register_trade::TradeRegisterTrade;
            serde_json::to_value(TradeRegisterTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeOpenTrade => {
            use crate::generated::gameevents::trade_open_trade::TradeOpenTrade;
            serde_json::to_value(TradeOpenTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeCloseTrade => {
            use crate::generated::gameevents::trade_close_trade::TradeCloseTrade;
            serde_json::to_value(TradeCloseTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeAddToTrade => {
            use crate::generated::gameevents::trade_add_to_trade::TradeAddToTrade;
            serde_json::to_value(TradeAddToTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeRemoveFromTrade => {
            use crate::generated::gameevents::trade_remove_from_trade::TradeRemoveFromTrade;
            serde_json::to_value(TradeRemoveFromTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeAcceptTrade => {
            use crate::generated::gameevents::trade_accept_trade::TradeAcceptTrade;
            serde_json::to_value(TradeAcceptTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeDeclineTrade => {
            use crate::generated::gameevents::trade_decline_trade::TradeDeclineTrade;
            serde_json::to_value(TradeDeclineTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeResetTrade => {
            use crate::generated::gameevents::trade_reset_trade::TradeResetTrade;
            serde_json::to_value(TradeResetTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeTradeFailure => {
            use crate::generated::gameevents::trade_trade_failure::TradeTradeFailure;
            serde_json::to_value(TradeTradeFailure::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::TradeClearTradeAcceptance => {
            use crate::generated::gameevents::trade_clear_trade_acceptance::TradeClearTradeAcceptance;
            serde_json::to_value(TradeClearTradeAcceptance::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseHouseProfile => {
            use crate::generated::gameevents::house_house_profile::HouseHouseProfile;
            serde_json::to_value(HouseHouseProfile::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseHouseData => {
            use crate::generated::gameevents::house_house_data::HouseHouseData;
            serde_json::to_value(HouseHouseData::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseHouseStatus => {
            use crate::generated::gameevents::house_house_status::HouseHouseStatus;
            serde_json::to_value(HouseHouseStatus::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseUpdateRentTime => {
            use crate::generated::gameevents::house_update_rent_time::HouseUpdateRentTime;
            serde_json::to_value(HouseUpdateRentTime::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseUpdateRentPayment => {
            use crate::generated::gameevents::house_update_rent_payment::HouseUpdateRentPayment;
            serde_json::to_value(HouseUpdateRentPayment::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseUpdateRestrictions => {
            use crate::generated::gameevents::house_update_restrictions::HouseUpdateRestrictions;
            serde_json::to_value(HouseUpdateRestrictions::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseUpdateHAR => {
            use crate::generated::gameevents::house_update_har::HouseUpdateHAR;
            serde_json::to_value(HouseUpdateHAR::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseHouseTransaction => {
            use crate::generated::gameevents::house_house_transaction::HouseHouseTransaction;
            serde_json::to_value(HouseHouseTransaction::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::ItemQueryItemManaResponse => {
            use crate::generated::gameevents::item_query_item_mana_response::ItemQueryItemManaResponse;
            serde_json::to_value(ItemQueryItemManaResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::HouseAvailableHouses => {
            use crate::generated::gameevents::house_available_houses::HouseAvailableHouses;
            serde_json::to_value(HouseAvailableHouses::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CharacterConfirmationRequest => {
            use crate::generated::gameevents::character_confirmation_request::CharacterConfirmationRequest;
            serde_json::to_value(CharacterConfirmationRequest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CharacterConfirmationDone => {
            use crate::generated::gameevents::character_confirmation_done::CharacterConfirmationDone;
            serde_json::to_value(CharacterConfirmationDone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AllegianceAllegianceLoginNotificationEvent => {
            use crate::generated::gameevents::allegiance_allegiance_login_notification_event::AllegianceAllegianceLoginNotificationEvent;
            serde_json::to_value(AllegianceAllegianceLoginNotificationEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AllegianceAllegianceInfoResponseEvent => {
            use crate::generated::gameevents::allegiance_allegiance_info_response_event::AllegianceAllegianceInfoResponseEvent;
            serde_json::to_value(AllegianceAllegianceInfoResponseEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::GameJoinGameResponse => {
            use crate::generated::gameevents::game_join_game_response::GameJoinGameResponse;
            serde_json::to_value(GameJoinGameResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::GameStartGame => {
            use crate::generated::gameevents::game_start_game::GameStartGame;
            serde_json::to_value(GameStartGame::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::GameMoveResponse => {
            use crate::generated::gameevents::game_move_response::GameMoveResponse;
            serde_json::to_value(GameMoveResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::GameOpponentTurn => {
            use crate::generated::gameevents::game_opponent_turn::GameOpponentTurn;
            serde_json::to_value(GameOpponentTurn::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::GameOpponentStalemateState => {
            use crate::generated::gameevents::game_opponent_stalemate_state::GameOpponentStalemateState;
            serde_json::to_value(GameOpponentStalemateState::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationWeenieError => {
            use crate::generated::gameevents::communication_weenie_error::CommunicationWeenieError;
            serde_json::to_value(CommunicationWeenieError::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationWeenieErrorWithString => {
            use crate::generated::gameevents::communication_weenie_error_with_string::CommunicationWeenieErrorWithString;
            serde_json::to_value(CommunicationWeenieErrorWithString::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::GameGameOver => {
            use crate::generated::gameevents::game_game_over::GameGameOver;
            serde_json::to_value(GameGameOver::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationChatRoomTracker => {
            use crate::generated::gameevents::communication_chat_room_tracker::CommunicationChatRoomTracker;
            serde_json::to_value(CommunicationChatRoomTracker::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AdminQueryPluginList => {
            use crate::generated::gameevents::admin_query_plugin_list::AdminQueryPluginList;
            serde_json::to_value(AdminQueryPluginList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AdminQueryPlugin => {
            use crate::generated::gameevents::admin_query_plugin::AdminQueryPlugin;
            serde_json::to_value(AdminQueryPlugin::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::AdminQueryPluginResponse2 => {
            use crate::generated::gameevents::admin_query_plugin_response2::AdminQueryPluginResponse2;
            serde_json::to_value(AdminQueryPluginResponse2::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::InventorySalvageOperationsResultData => {
            use crate::generated::gameevents::inventory_salvage_operations_result_data::InventorySalvageOperationsResultData;
            serde_json::to_value(InventorySalvageOperationsResultData::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationHearDirectSpeech => {
            use crate::generated::gameevents::communication_hear_direct_speech::CommunicationHearDirectSpeech;
            serde_json::to_value(CommunicationHearDirectSpeech::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipFullUpdate => {
            use crate::generated::gameevents::fellowship_full_update::FellowshipFullUpdate;
            serde_json::to_value(FellowshipFullUpdate::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipDisband => {
            use crate::generated::gameevents::fellowship_disband::FellowshipDisband;
            serde_json::to_value(FellowshipDisband::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::FellowshipUpdateFellow => {
            use crate::generated::gameevents::fellowship_update_fellow::FellowshipUpdateFellow;
            serde_json::to_value(FellowshipUpdateFellow::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicUpdateSpell => {
            use crate::generated::gameevents::magic_update_spell::MagicUpdateSpell;
            serde_json::to_value(MagicUpdateSpell::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicUpdateEnchantment => {
            use crate::generated::gameevents::magic_update_enchantment::MagicUpdateEnchantment;
            serde_json::to_value(MagicUpdateEnchantment::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicRemoveEnchantment => {
            use crate::generated::gameevents::magic_remove_enchantment::MagicRemoveEnchantment;
            serde_json::to_value(MagicRemoveEnchantment::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicUpdateMultipleEnchantments => {
            use crate::generated::gameevents::magic_update_multiple_enchantments::MagicUpdateMultipleEnchantments;
            serde_json::to_value(MagicUpdateMultipleEnchantments::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicRemoveMultipleEnchantments => {
            use crate::generated::gameevents::magic_remove_multiple_enchantments::MagicRemoveMultipleEnchantments;
            serde_json::to_value(MagicRemoveMultipleEnchantments::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicPurgeEnchantments => {
            use crate::generated::gameevents::magic_purge_enchantments::MagicPurgeEnchantments;
            serde_json::to_value(MagicPurgeEnchantments::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicDispelEnchantment => {
            use crate::generated::gameevents::magic_dispel_enchantment::MagicDispelEnchantment;
            serde_json::to_value(MagicDispelEnchantment::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicDispelMultipleEnchantments => {
            use crate::generated::gameevents::magic_dispel_multiple_enchantments::MagicDispelMultipleEnchantments;
            serde_json::to_value(MagicDispelMultipleEnchantments::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MiscPortalStormBrewing => {
            use crate::generated::gameevents::misc_portal_storm_brewing::MiscPortalStormBrewing;
            serde_json::to_value(MiscPortalStormBrewing::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MiscPortalStormImminent => {
            use crate::generated::gameevents::misc_portal_storm_imminent::MiscPortalStormImminent;
            serde_json::to_value(MiscPortalStormImminent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MiscPortalStorm => {
            use crate::generated::gameevents::misc_portal_storm::MiscPortalStorm;
            serde_json::to_value(MiscPortalStorm::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MiscPortalStormSubsided => {
            use crate::generated::gameevents::misc_portal_storm_subsided::MiscPortalStormSubsided;
            serde_json::to_value(MiscPortalStormSubsided::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::CommunicationTransientString => {
            use crate::generated::gameevents::communication_transient_string::CommunicationTransientString;
            serde_json::to_value(CommunicationTransientString::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::MagicPurgeBadEnchantments => {
            use crate::generated::gameevents::magic_purge_bad_enchantments::MagicPurgeBadEnchantments;
            serde_json::to_value(MagicPurgeBadEnchantments::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::SocialSendClientContractTrackerTable => {
            use crate::generated::gameevents::social_send_client_contract_tracker_table::SocialSendClientContractTrackerTable;
            serde_json::to_value(SocialSendClientContractTrackerTable::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameEvent::SocialSendClientContractTracker => {
            use crate::generated::gameevents::social_send_client_contract_tracker::SocialSendClientContractTracker;
            serde_json::to_value(SocialSendClientContractTracker::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        _ => Ok(json!({})),
    };

    let mut result = match event_data {
        Ok(data) => data,
        Err(e) => return Ok(json!({
            "objectId": object_id,
            "sequence": sequence,
            "eventType": event_type_value,
            "error": format!("Failed to parse: {}", e)
        })),
    };

    // Add header fields to the result
    if let serde_json::Value::Object(ref mut map) = result {
        map.insert("objectId".to_string(), serde_json::json!(object_id));
        map.insert("sequence".to_string(), serde_json::json!(sequence));
        map.insert("eventType".to_string(), serde_json::json!(event_type_value));
    }

    Ok(result)
}

fn parse_game_action(
    cursor: &mut Cursor<&[u8]>,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Store initial position before reading header
    let _initial_pos = cursor.position();
    
    // Read header fields to determine the action type (as per Chorizite pattern)
    let sequence = crate::readers::read_u32(&mut Box::new(cursor.clone()))?;
    let action_type_value = crate::readers::read_u32(&mut Box::new(cursor.clone()))?;
    
    let action_type = match GameAction::try_from(action_type_value) {
        Ok(at) => at,
        Err(_) => return Ok(json!({
            "sequence": sequence,
            "actionType": action_type_value,
            "error": "Unknown action type"
        })),
    };

    // Backtrack to allow the specific struct to read the full payload (as per Chorizite pattern)
    let pos = cursor.position();
    if pos >= 8 {
        cursor.seek(std::io::SeekFrom::Current(-8))?;  // Backtrack 2 u32s = 8 bytes
    } else {
        cursor.set_position(0);
    }
    
    // Create a reader from the current cursor position to parse the full payload
    let mut reader: Box<dyn ACReader> = Box::new(cursor.clone());
    
    // Parse using the specific action struct (which will read header + payload)
    let action_data: Result<serde_json::Value, Box<dyn std::error::Error>> = match action_type {
        GameAction::CharacterPlayerOptionChangedEvent => {
            use crate::generated::gameactions::character_player_option_changed_event::CharacterPlayerOptionChangedEvent;
            serde_json::to_value(CharacterPlayerOptionChangedEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CombatTargetedMeleeAttack => {
            use crate::generated::gameactions::combat_targeted_melee_attack::CombatTargetedMeleeAttack;
            serde_json::to_value(CombatTargetedMeleeAttack::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CombatTargetedMissileAttack => {
            use crate::generated::gameactions::combat_targeted_missile_attack::CombatTargetedMissileAttack;
            serde_json::to_value(CombatTargetedMissileAttack::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationSetAFKMode => {
            use crate::generated::gameactions::communication_set_afk_mode::CommunicationSetAFKMode;
            serde_json::to_value(CommunicationSetAFKMode::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationSetAFKMessage => {
            use crate::generated::gameactions::communication_set_afk_message::CommunicationSetAFKMessage;
            serde_json::to_value(CommunicationSetAFKMessage::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationTalk => {
            use crate::generated::gameactions::communication_talk::CommunicationTalk;
            serde_json::to_value(CommunicationTalk::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::SocialRemoveFriend => {
            use crate::generated::gameactions::social_remove_friend::SocialRemoveFriend;
            serde_json::to_value(SocialRemoveFriend::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::SocialAddFriend => {
            use crate::generated::gameactions::social_add_friend::SocialAddFriend;
            serde_json::to_value(SocialAddFriend::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryPutItemInContainer => {
            use crate::generated::gameactions::inventory_put_item_in_container::InventoryPutItemInContainer;
            serde_json::to_value(InventoryPutItemInContainer::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryGetAndWieldItem => {
            use crate::generated::gameactions::inventory_get_and_wield_item::InventoryGetAndWieldItem;
            serde_json::to_value(InventoryGetAndWieldItem::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryDropItem => {
            use crate::generated::gameactions::inventory_drop_item::InventoryDropItem;
            serde_json::to_value(InventoryDropItem::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceSwearAllegiance => {
            use crate::generated::gameactions::allegiance_swear_allegiance::AllegianceSwearAllegiance;
            serde_json::to_value(AllegianceSwearAllegiance::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceBreakAllegiance => {
            use crate::generated::gameactions::allegiance_break_allegiance::AllegianceBreakAllegiance;
            serde_json::to_value(AllegianceBreakAllegiance::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceUpdateRequest => {
            use crate::generated::gameactions::allegiance_update_request::AllegianceUpdateRequest;
            serde_json::to_value(AllegianceUpdateRequest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::SocialClearFriends => {
            use crate::generated::gameactions::social_clear_friends::SocialClearFriends;
            serde_json::to_value(SocialClearFriends::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterTeleToPKLArena => {
            use crate::generated::gameactions::character_tele_to_pkl_arena::CharacterTeleToPKLArena;
            serde_json::to_value(CharacterTeleToPKLArena::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterTeleToPKArena => {
            use crate::generated::gameactions::character_tele_to_pk_arena::CharacterTeleToPKArena;
            serde_json::to_value(CharacterTeleToPKArena::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::SocialSetDisplayCharacterTitle => {
            use crate::generated::gameactions::social_set_display_character_title::SocialSetDisplayCharacterTitle;
            serde_json::to_value(SocialSetDisplayCharacterTitle::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceQueryAllegianceName => {
            use crate::generated::gameactions::allegiance_query_allegiance_name::AllegianceQueryAllegianceName;
            serde_json::to_value(AllegianceQueryAllegianceName::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceClearAllegianceName => {
            use crate::generated::gameactions::allegiance_clear_allegiance_name::AllegianceClearAllegianceName;
            serde_json::to_value(AllegianceClearAllegianceName::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationTalkDirect => {
            use crate::generated::gameactions::communication_talk_direct::CommunicationTalkDirect;
            serde_json::to_value(CommunicationTalkDirect::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceSetAllegianceName => {
            use crate::generated::gameactions::allegiance_set_allegiance_name::AllegianceSetAllegianceName;
            serde_json::to_value(AllegianceSetAllegianceName::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryUseWithTargetEvent => {
            use crate::generated::gameactions::inventory_use_with_target_event::InventoryUseWithTargetEvent;
            serde_json::to_value(InventoryUseWithTargetEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryUseEvent => {
            use crate::generated::gameactions::inventory_use_event::InventoryUseEvent;
            serde_json::to_value(InventoryUseEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceSetAllegianceOfficer => {
            use crate::generated::gameactions::allegiance_set_allegiance_officer::AllegianceSetAllegianceOfficer;
            serde_json::to_value(AllegianceSetAllegianceOfficer::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceSetAllegianceOfficerTitle => {
            use crate::generated::gameactions::allegiance_set_allegiance_officer_title::AllegianceSetAllegianceOfficerTitle;
            serde_json::to_value(AllegianceSetAllegianceOfficerTitle::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceListAllegianceOfficerTitles => {
            use crate::generated::gameactions::allegiance_list_allegiance_officer_titles::AllegianceListAllegianceOfficerTitles;
            serde_json::to_value(AllegianceListAllegianceOfficerTitles::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceClearAllegianceOfficerTitles => {
            use crate::generated::gameactions::allegiance_clear_allegiance_officer_titles::AllegianceClearAllegianceOfficerTitles;
            serde_json::to_value(AllegianceClearAllegianceOfficerTitles::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceDoAllegianceLockAction => {
            use crate::generated::gameactions::allegiance_do_allegiance_lock_action::AllegianceDoAllegianceLockAction;
            serde_json::to_value(AllegianceDoAllegianceLockAction::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceSetAllegianceApprovedVassal => {
            use crate::generated::gameactions::allegiance_set_allegiance_approved_vassal::AllegianceSetAllegianceApprovedVassal;
            serde_json::to_value(AllegianceSetAllegianceApprovedVassal::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceAllegianceChatGag => {
            use crate::generated::gameactions::allegiance_allegiance_chat_gag::AllegianceAllegianceChatGag;
            serde_json::to_value(AllegianceAllegianceChatGag::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceDoAllegianceHouseAction => {
            use crate::generated::gameactions::allegiance_do_allegiance_house_action::AllegianceDoAllegianceHouseAction;
            serde_json::to_value(AllegianceDoAllegianceHouseAction::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TrainTrainAttribute2nd => {
            use crate::generated::gameactions::train_train_attribute2nd::TrainTrainAttribute2nd;
            serde_json::to_value(TrainTrainAttribute2nd::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TrainTrainAttribute => {
            use crate::generated::gameactions::train_train_attribute::TrainTrainAttribute;
            serde_json::to_value(TrainTrainAttribute::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TrainTrainSkill => {
            use crate::generated::gameactions::train_train_skill::TrainTrainSkill;
            serde_json::to_value(TrainTrainSkill::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TrainTrainSkillAdvancementClass => {
            use crate::generated::gameactions::train_train_skill_advancement_class::TrainTrainSkillAdvancementClass;
            serde_json::to_value(TrainTrainSkillAdvancementClass::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MagicCastUntargetedSpell => {
            use crate::generated::gameactions::magic_cast_untargeted_spell::MagicCastUntargetedSpell;
            serde_json::to_value(MagicCastUntargetedSpell::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MagicCastTargetedSpell => {
            use crate::generated::gameactions::magic_cast_targeted_spell::MagicCastTargetedSpell;
            serde_json::to_value(MagicCastTargetedSpell::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CombatChangeCombatMode => {
            use crate::generated::gameactions::combat_change_combat_mode::CombatChangeCombatMode;
            serde_json::to_value(CombatChangeCombatMode::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryStackableMerge => {
            use crate::generated::gameactions::inventory_stackable_merge::InventoryStackableMerge;
            serde_json::to_value(InventoryStackableMerge::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryStackableSplitToContainer => {
            use crate::generated::gameactions::inventory_stackable_split_to_container::InventoryStackableSplitToContainer;
            serde_json::to_value(InventoryStackableSplitToContainer::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryStackableSplitTo3D => {
            use crate::generated::gameactions::inventory_stackable_split_to3d::InventoryStackableSplitTo3D;
            serde_json::to_value(InventoryStackableSplitTo3D::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationModifyCharacterSquelch => {
            use crate::generated::gameactions::communication_modify_character_squelch::CommunicationModifyCharacterSquelch;
            serde_json::to_value(CommunicationModifyCharacterSquelch::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationModifyAccountSquelch => {
            use crate::generated::gameactions::communication_modify_account_squelch::CommunicationModifyAccountSquelch;
            serde_json::to_value(CommunicationModifyAccountSquelch::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationModifyGlobalSquelch => {
            use crate::generated::gameactions::communication_modify_global_squelch::CommunicationModifyGlobalSquelch;
            serde_json::to_value(CommunicationModifyGlobalSquelch::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationTalkDirectByName => {
            use crate::generated::gameactions::communication_talk_direct_by_name::CommunicationTalkDirectByName;
            serde_json::to_value(CommunicationTalkDirectByName::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::VendorBuy => {
            use crate::generated::gameactions::vendor_buy::VendorBuy;
            serde_json::to_value(VendorBuy::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::VendorSell => {
            use crate::generated::gameactions::vendor_sell::VendorSell;
            serde_json::to_value(VendorSell::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterTeleToLifestone => {
            use crate::generated::gameactions::character_tele_to_lifestone::CharacterTeleToLifestone;
            serde_json::to_value(CharacterTeleToLifestone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterLoginCompleteNotification => {
            use crate::generated::gameactions::character_login_complete_notification::CharacterLoginCompleteNotification;
            serde_json::to_value(CharacterLoginCompleteNotification::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipCreate => {
            use crate::generated::gameactions::fellowship_create::FellowshipCreate;
            serde_json::to_value(FellowshipCreate::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipQuit => {
            use crate::generated::gameactions::fellowship_quit::FellowshipQuit;
            serde_json::to_value(FellowshipQuit::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipDismiss => {
            use crate::generated::gameactions::fellowship_dismiss::FellowshipDismiss;
            serde_json::to_value(FellowshipDismiss::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipRecruit => {
            use crate::generated::gameactions::fellowship_recruit::FellowshipRecruit;
            serde_json::to_value(FellowshipRecruit::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipUpdateRequest => {
            use crate::generated::gameactions::fellowship_update_request::FellowshipUpdateRequest;
            serde_json::to_value(FellowshipUpdateRequest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::WritingBookAddPage => {
            use crate::generated::gameactions::writing_book_add_page::WritingBookAddPage;
            serde_json::to_value(WritingBookAddPage::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::WritingBookModifyPage => {
            use crate::generated::gameactions::writing_book_modify_page::WritingBookModifyPage;
            serde_json::to_value(WritingBookModifyPage::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::WritingBookData => {
            use crate::generated::gameactions::writing_book_data::WritingBookData;
            serde_json::to_value(WritingBookData::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::WritingBookDeletePage => {
            use crate::generated::gameactions::writing_book_delete_page::WritingBookDeletePage;
            serde_json::to_value(WritingBookDeletePage::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::WritingBookPageData => {
            use crate::generated::gameactions::writing_book_page_data::WritingBookPageData;
            serde_json::to_value(WritingBookPageData::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::WritingSetInscription => {
            use crate::generated::gameactions::writing_set_inscription::WritingSetInscription;
            serde_json::to_value(WritingSetInscription::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::ItemAppraise => {
            use crate::generated::gameactions::item_appraise::ItemAppraise;
            serde_json::to_value(ItemAppraise::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryGiveObjectRequest => {
            use crate::generated::gameactions::inventory_give_object_request::InventoryGiveObjectRequest;
            serde_json::to_value(InventoryGiveObjectRequest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AdvocateTeleport => {
            use crate::generated::gameactions::advocate_teleport::AdvocateTeleport;
            serde_json::to_value(AdvocateTeleport::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterAbuseLogRequest => {
            use crate::generated::gameactions::character_abuse_log_request::CharacterAbuseLogRequest;
            serde_json::to_value(CharacterAbuseLogRequest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationAddToChannel => {
            use crate::generated::gameactions::communication_add_to_channel::CommunicationAddToChannel;
            serde_json::to_value(CommunicationAddToChannel::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationRemoveFromChannel => {
            use crate::generated::gameactions::communication_remove_from_channel::CommunicationRemoveFromChannel;
            serde_json::to_value(CommunicationRemoveFromChannel::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationChannelBroadcast => {
            use crate::generated::gameactions::communication_channel_broadcast::CommunicationChannelBroadcast;
            serde_json::to_value(CommunicationChannelBroadcast::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationChannelList => {
            use crate::generated::gameactions::communication_channel_list::CommunicationChannelList;
            serde_json::to_value(CommunicationChannelList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationChannelIndex => {
            use crate::generated::gameactions::communication_channel_index::CommunicationChannelIndex;
            serde_json::to_value(CommunicationChannelIndex::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryNoLongerViewingContents => {
            use crate::generated::gameactions::inventory_no_longer_viewing_contents::InventoryNoLongerViewingContents;
            serde_json::to_value(InventoryNoLongerViewingContents::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryStackableSplitToWield => {
            use crate::generated::gameactions::inventory_stackable_split_to_wield::InventoryStackableSplitToWield;
            serde_json::to_value(InventoryStackableSplitToWield::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterAddShortCut => {
            use crate::generated::gameactions::character_add_short_cut::CharacterAddShortCut;
            serde_json::to_value(CharacterAddShortCut::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterRemoveShortCut => {
            use crate::generated::gameactions::character_remove_short_cut::CharacterRemoveShortCut;
            serde_json::to_value(CharacterRemoveShortCut::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterCharacterOptionsEvent => {
            use crate::generated::gameactions::character_character_options_event::CharacterCharacterOptionsEvent;
            serde_json::to_value(CharacterCharacterOptionsEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MagicRemoveSpell => {
            use crate::generated::gameactions::magic_remove_spell::MagicRemoveSpell;
            serde_json::to_value(MagicRemoveSpell::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CombatCancelAttack => {
            use crate::generated::gameactions::combat_cancel_attack::CombatCancelAttack;
            serde_json::to_value(CombatCancelAttack::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CombatQueryHealth => {
            use crate::generated::gameactions::combat_query_health::CombatQueryHealth;
            serde_json::to_value(CombatQueryHealth::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterQueryAge => {
            use crate::generated::gameactions::character_query_age::CharacterQueryAge;
            serde_json::to_value(CharacterQueryAge::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterQueryBirth => {
            use crate::generated::gameactions::character_query_birth::CharacterQueryBirth;
            serde_json::to_value(CharacterQueryBirth::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationEmote => {
            use crate::generated::gameactions::communication_emote::CommunicationEmote;
            serde_json::to_value(CommunicationEmote::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CommunicationSoulEmote => {
            use crate::generated::gameactions::communication_soul_emote::CommunicationSoulEmote;
            serde_json::to_value(CommunicationSoulEmote::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterAddSpellFavorite => {
            use crate::generated::gameactions::character_add_spell_favorite::CharacterAddSpellFavorite;
            serde_json::to_value(CharacterAddSpellFavorite::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterRemoveSpellFavorite => {
            use crate::generated::gameactions::character_remove_spell_favorite::CharacterRemoveSpellFavorite;
            serde_json::to_value(CharacterRemoveSpellFavorite::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterRequestPing => {
            use crate::generated::gameactions::character_request_ping::CharacterRequestPing;
            serde_json::to_value(CharacterRequestPing::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TradeOpenTradeNegotiations => {
            use crate::generated::gameactions::trade_open_trade_negotiations::TradeOpenTradeNegotiations;
            serde_json::to_value(TradeOpenTradeNegotiations::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TradeCloseTradeNegotiations => {
            use crate::generated::gameactions::trade_close_trade_negotiations::TradeCloseTradeNegotiations;
            serde_json::to_value(TradeCloseTradeNegotiations::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TradeAddToTrade => {
            use crate::generated::gameactions::trade_add_to_trade::TradeAddToTrade;
            serde_json::to_value(TradeAddToTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TradeAcceptTrade => {
            use crate::generated::gameactions::trade_accept_trade::TradeAcceptTrade;
            serde_json::to_value(TradeAcceptTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TradeDeclineTrade => {
            use crate::generated::gameactions::trade_decline_trade::TradeDeclineTrade;
            serde_json::to_value(TradeDeclineTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::TradeResetTrade => {
            use crate::generated::gameactions::trade_reset_trade::TradeResetTrade;
            serde_json::to_value(TradeResetTrade::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterClearPlayerConsentList => {
            use crate::generated::gameactions::character_clear_player_consent_list::CharacterClearPlayerConsentList;
            serde_json::to_value(CharacterClearPlayerConsentList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterDisplayPlayerConsentList => {
            use crate::generated::gameactions::character_display_player_consent_list::CharacterDisplayPlayerConsentList;
            serde_json::to_value(CharacterDisplayPlayerConsentList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterRemoveFromPlayerConsentList => {
            use crate::generated::gameactions::character_remove_from_player_consent_list::CharacterRemoveFromPlayerConsentList;
            serde_json::to_value(CharacterRemoveFromPlayerConsentList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterAddPlayerPermission => {
            use crate::generated::gameactions::character_add_player_permission::CharacterAddPlayerPermission;
            serde_json::to_value(CharacterAddPlayerPermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseBuyHouse => {
            use crate::generated::gameactions::house_buy_house::HouseBuyHouse;
            serde_json::to_value(HouseBuyHouse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseQueryHouse => {
            use crate::generated::gameactions::house_query_house::HouseQueryHouse;
            serde_json::to_value(HouseQueryHouse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseAbandonHouse => {
            use crate::generated::gameactions::house_abandon_house::HouseAbandonHouse;
            serde_json::to_value(HouseAbandonHouse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterRemovePlayerPermission => {
            use crate::generated::gameactions::character_remove_player_permission::CharacterRemovePlayerPermission;
            serde_json::to_value(CharacterRemovePlayerPermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseRentHouse => {
            use crate::generated::gameactions::house_rent_house::HouseRentHouse;
            serde_json::to_value(HouseRentHouse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterSetDesiredComponentLevel => {
            use crate::generated::gameactions::character_set_desired_component_level::CharacterSetDesiredComponentLevel;
            serde_json::to_value(CharacterSetDesiredComponentLevel::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseAddPermanentGuest => {
            use crate::generated::gameactions::house_add_permanent_guest::HouseAddPermanentGuest;
            serde_json::to_value(HouseAddPermanentGuest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseRemovePermanentGuest => {
            use crate::generated::gameactions::house_remove_permanent_guest::HouseRemovePermanentGuest;
            serde_json::to_value(HouseRemovePermanentGuest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseSetOpenHouseStatus => {
            use crate::generated::gameactions::house_set_open_house_status::HouseSetOpenHouseStatus;
            serde_json::to_value(HouseSetOpenHouseStatus::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseChangeStoragePermission => {
            use crate::generated::gameactions::house_change_storage_permission::HouseChangeStoragePermission;
            serde_json::to_value(HouseChangeStoragePermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseBootSpecificHouseGuest => {
            use crate::generated::gameactions::house_boot_specific_house_guest::HouseBootSpecificHouseGuest;
            serde_json::to_value(HouseBootSpecificHouseGuest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseRemoveAllStoragePermission => {
            use crate::generated::gameactions::house_remove_all_storage_permission::HouseRemoveAllStoragePermission;
            serde_json::to_value(HouseRemoveAllStoragePermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseRequestFullGuestList => {
            use crate::generated::gameactions::house_request_full_guest_list::HouseRequestFullGuestList;
            serde_json::to_value(HouseRequestFullGuestList::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceSetMotd => {
            use crate::generated::gameactions::allegiance_set_motd::AllegianceSetMotd;
            serde_json::to_value(AllegianceSetMotd::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceQueryMotd => {
            use crate::generated::gameactions::allegiance_query_motd::AllegianceQueryMotd;
            serde_json::to_value(AllegianceQueryMotd::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceClearMotd => {
            use crate::generated::gameactions::allegiance_clear_motd::AllegianceClearMotd;
            serde_json::to_value(AllegianceClearMotd::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseQueryLord => {
            use crate::generated::gameactions::house_query_lord::HouseQueryLord;
            serde_json::to_value(HouseQueryLord::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseAddAllStoragePermission => {
            use crate::generated::gameactions::house_add_all_storage_permission::HouseAddAllStoragePermission;
            serde_json::to_value(HouseAddAllStoragePermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseRemoveAllPermanentGuests => {
            use crate::generated::gameactions::house_remove_all_permanent_guests::HouseRemoveAllPermanentGuests;
            serde_json::to_value(HouseRemoveAllPermanentGuests::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseBootEveryone => {
            use crate::generated::gameactions::house_boot_everyone::HouseBootEveryone;
            serde_json::to_value(HouseBootEveryone::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseTeleToHouse => {
            use crate::generated::gameactions::house_tele_to_house::HouseTeleToHouse;
            serde_json::to_value(HouseTeleToHouse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::ItemQueryItemMana => {
            use crate::generated::gameactions::item_query_item_mana::ItemQueryItemMana;
            serde_json::to_value(ItemQueryItemMana::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseSetHooksVisibility => {
            use crate::generated::gameactions::house_set_hooks_visibility::HouseSetHooksVisibility;
            serde_json::to_value(HouseSetHooksVisibility::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseModifyAllegianceGuestPermission => {
            use crate::generated::gameactions::house_modify_allegiance_guest_permission::HouseModifyAllegianceGuestPermission;
            serde_json::to_value(HouseModifyAllegianceGuestPermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseModifyAllegianceStoragePermission => {
            use crate::generated::gameactions::house_modify_allegiance_storage_permission::HouseModifyAllegianceStoragePermission;
            serde_json::to_value(HouseModifyAllegianceStoragePermission::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::GameJoin => {
            use crate::generated::gameactions::game_join::GameJoin;
            serde_json::to_value(GameJoin::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::GameQuit => {
            use crate::generated::gameactions::game_quit::GameQuit;
            serde_json::to_value(GameQuit::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::GameMove => {
            use crate::generated::gameactions::game_move::GameMove;
            serde_json::to_value(GameMove::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::GameMovePass => {
            use crate::generated::gameactions::game_move_pass::GameMovePass;
            serde_json::to_value(GameMovePass::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::GameStalemate => {
            use crate::generated::gameactions::game_stalemate::GameStalemate;
            serde_json::to_value(GameStalemate::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseListAvailableHouses => {
            use crate::generated::gameactions::house_list_available_houses::HouseListAvailableHouses;
            serde_json::to_value(HouseListAvailableHouses::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterConfirmationResponse => {
            use crate::generated::gameactions::character_confirmation_response::CharacterConfirmationResponse;
            serde_json::to_value(CharacterConfirmationResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceBreakAllegianceBoot => {
            use crate::generated::gameactions::allegiance_break_allegiance_boot::AllegianceBreakAllegianceBoot;
            serde_json::to_value(AllegianceBreakAllegianceBoot::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::HouseTeleToMansion => {
            use crate::generated::gameactions::house_tele_to_mansion::HouseTeleToMansion;
            serde_json::to_value(HouseTeleToMansion::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterSuicide => {
            use crate::generated::gameactions::character_suicide::CharacterSuicide;
            serde_json::to_value(CharacterSuicide::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceAllegianceInfoRequest => {
            use crate::generated::gameactions::allegiance_allegiance_info_request::AllegianceAllegianceInfoRequest;
            serde_json::to_value(AllegianceAllegianceInfoRequest::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::InventoryCreateTinkeringTool => {
            use crate::generated::gameactions::inventory_create_tinkering_tool::InventoryCreateTinkeringTool;
            serde_json::to_value(InventoryCreateTinkeringTool::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterSpellbookFilterEvent => {
            use crate::generated::gameactions::character_spellbook_filter_event::CharacterSpellbookFilterEvent;
            serde_json::to_value(CharacterSpellbookFilterEvent::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterTeleToMarketplace => {
            use crate::generated::gameactions::character_tele_to_marketplace::CharacterTeleToMarketplace;
            serde_json::to_value(CharacterTeleToMarketplace::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterEnterPKLite => {
            use crate::generated::gameactions::character_enter_pk_lite::CharacterEnterPKLite;
            serde_json::to_value(CharacterEnterPKLite::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipAssignNewLeader => {
            use crate::generated::gameactions::fellowship_assign_new_leader::FellowshipAssignNewLeader;
            serde_json::to_value(FellowshipAssignNewLeader::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::FellowshipChangeFellowOpeness => {
            use crate::generated::gameactions::fellowship_change_fellow_openess::FellowshipChangeFellowOpeness;
            serde_json::to_value(FellowshipChangeFellowOpeness::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceAllegianceChatBoot => {
            use crate::generated::gameactions::allegiance_allegiance_chat_boot::AllegianceAllegianceChatBoot;
            serde_json::to_value(AllegianceAllegianceChatBoot::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceAddAllegianceBan => {
            use crate::generated::gameactions::allegiance_add_allegiance_ban::AllegianceAddAllegianceBan;
            serde_json::to_value(AllegianceAddAllegianceBan::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceRemoveAllegianceBan => {
            use crate::generated::gameactions::allegiance_remove_allegiance_ban::AllegianceRemoveAllegianceBan;
            serde_json::to_value(AllegianceRemoveAllegianceBan::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceListAllegianceBans => {
            use crate::generated::gameactions::allegiance_list_allegiance_bans::AllegianceListAllegianceBans;
            serde_json::to_value(AllegianceListAllegianceBans::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceRemoveAllegianceOfficer => {
            use crate::generated::gameactions::allegiance_remove_allegiance_officer::AllegianceRemoveAllegianceOfficer;
            serde_json::to_value(AllegianceRemoveAllegianceOfficer::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceListAllegianceOfficers => {
            use crate::generated::gameactions::allegiance_list_allegiance_officers::AllegianceListAllegianceOfficers;
            serde_json::to_value(AllegianceListAllegianceOfficers::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceClearAllegianceOfficers => {
            use crate::generated::gameactions::allegiance_clear_allegiance_officers::AllegianceClearAllegianceOfficers;
            serde_json::to_value(AllegianceClearAllegianceOfficers::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AllegianceRecallAllegianceHometown => {
            use crate::generated::gameactions::allegiance_recall_allegiance_hometown::AllegianceRecallAllegianceHometown;
            serde_json::to_value(AllegianceRecallAllegianceHometown::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AdminQueryPluginListResponse => {
            use crate::generated::gameactions::admin_query_plugin_list_response::AdminQueryPluginListResponse;
            serde_json::to_value(AdminQueryPluginListResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::AdminQueryPluginResponse => {
            use crate::generated::gameactions::admin_query_plugin_response::AdminQueryPluginResponse;
            serde_json::to_value(AdminQueryPluginResponse::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::CharacterFinishBarber => {
            use crate::generated::gameactions::character_finish_barber::CharacterFinishBarber;
            serde_json::to_value(CharacterFinishBarber::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::SocialAbandonContract => {
            use crate::generated::gameactions::social_abandon_contract::SocialAbandonContract;
            serde_json::to_value(SocialAbandonContract::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementJump => {
            use crate::generated::gameactions::movement_jump::MovementJump;
            serde_json::to_value(MovementJump::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementMoveToState => {
            use crate::generated::gameactions::movement_move_to_state::MovementMoveToState;
            serde_json::to_value(MovementMoveToState::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementDoMovementCommand => {
            use crate::generated::gameactions::movement_do_movement_command::MovementDoMovementCommand;
            serde_json::to_value(MovementDoMovementCommand::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementStopMovementCommand => {
            use crate::generated::gameactions::movement_stop_movement_command::MovementStopMovementCommand;
            serde_json::to_value(MovementStopMovementCommand::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementAutonomyLevel => {
            use crate::generated::gameactions::movement_autonomy_level::MovementAutonomyLevel;
            serde_json::to_value(MovementAutonomyLevel::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementAutonomousPosition => {
            use crate::generated::gameactions::movement_autonomous_position::MovementAutonomousPosition;
            serde_json::to_value(MovementAutonomousPosition::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        GameAction::MovementJumpNonAutonomous => {
            use crate::generated::gameactions::movement_jump_non_autonomous::MovementJumpNonAutonomous;
            serde_json::to_value(MovementJumpNonAutonomous::read(&mut reader)?).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        }
        _ => Ok(json!({})),
    };

    let mut result = match action_data {
        Ok(data) => data,
        Err(e) => return Ok(json!({
            "sequence": sequence,
            "actionType": action_type_value,
            "error": format!("Failed to parse: {}", e)
        })),
    };

    // Add header fields to the result
    if let serde_json::Value::Object(ref mut map) = result {
        map.insert("sequence".to_string(), serde_json::json!(sequence));
        map.insert("actionType".to_string(), serde_json::json!(action_type_value));
    }

    Ok(result)
}
