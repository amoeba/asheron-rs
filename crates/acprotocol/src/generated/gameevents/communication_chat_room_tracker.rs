use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChatRoomTracker")]
pub struct CommunicationChatRoomTracker {
    #[serde(rename = "AllegianceRoomId")]
    pub allegiance_room_id: u32,
    #[serde(rename = "GeneralChatRoomId")]
    pub general_chat_room_id: u32,
    #[serde(rename = "TradeChatRoomId")]
    pub trade_chat_room_id: u32,
    #[serde(rename = "LFGChatRoomId")]
    pub lfg_chat_room_id: u32,
    #[serde(rename = "RoleplayChatRoomId")]
    pub roleplay_chat_room_id: u32,
    #[serde(rename = "OlthoiChatRoomId")]
    pub olthoi_chat_room_id: u32,
    #[serde(rename = "SocietyChatRoomId")]
    pub society_chat_room_id: u32,
    #[serde(rename = "SocietyCelestialHandChatRoomId")]
    pub society_celestial_hand_chat_room_id: u32,
    #[serde(rename = "SocietyEldrichWebChatRoomId")]
    pub society_eldrich_web_chat_room_id: u32,
    #[serde(rename = "SocietyRadiantBloodChatRoomId")]
    pub society_radiant_blood_chat_room_id: u32,
}

impl CommunicationChatRoomTracker {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let allegiance_room_id = read_u32(reader)?;
        let general_chat_room_id = read_u32(reader)?;
        let trade_chat_room_id = read_u32(reader)?;
        let lfg_chat_room_id = read_u32(reader)?;
        let roleplay_chat_room_id = read_u32(reader)?;
        let olthoi_chat_room_id = read_u32(reader)?;
        let society_chat_room_id = read_u32(reader)?;
        let society_celestial_hand_chat_room_id = read_u32(reader)?;
        let society_eldrich_web_chat_room_id = read_u32(reader)?;
        let society_radiant_blood_chat_room_id = read_u32(reader)?;

        Ok(Self {
            allegiance_room_id,
            general_chat_room_id,
            trade_chat_room_id,
            lfg_chat_room_id,
            roleplay_chat_room_id,
            olthoi_chat_room_id,
            society_chat_room_id,
            society_celestial_hand_chat_room_id,
            society_eldrich_web_chat_room_id,
            society_radiant_blood_chat_room_id,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChatRoomTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChatRoomTracker::read(reader)
    }
}

