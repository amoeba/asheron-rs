use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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

impl crate::readers::ACDataType for CommunicationChatRoomTracker {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationChatRoomTracker").entered();

        #[cfg(feature = "tracing")]
        let _field_span_allegiance_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AllegianceRoomId", position = pos).entered()
        };
        let allegiance_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_allegiance_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_general_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GeneralChatRoomId", position = pos).entered()
        };
        let general_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_general_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_trade_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TradeChatRoomId", position = pos).entered()
        };
        let trade_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_trade_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_lfg_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "LFGChatRoomId", position = pos).entered()
        };
        let lfg_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_lfg_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_roleplay_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RoleplayChatRoomId", position = pos).entered()
        };
        let roleplay_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_roleplay_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_olthoi_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OlthoiChatRoomId", position = pos).entered()
        };
        let olthoi_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_olthoi_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_society_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SocietyChatRoomId", position = pos).entered()
        };
        let society_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_society_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_society_celestial_hand_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SocietyCelestialHandChatRoomId", position = pos).entered()
        };
        let society_celestial_hand_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_society_celestial_hand_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_society_eldrich_web_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SocietyEldrichWebChatRoomId", position = pos).entered()
        };
        let society_eldrich_web_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_society_eldrich_web_chat_room_id);
        #[cfg(feature = "tracing")]
        let _field_span_society_radiant_blood_chat_room_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SocietyRadiantBloodChatRoomId", position = pos).entered()
        };
        let society_radiant_blood_chat_room_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_society_radiant_blood_chat_room_id);

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

impl crate::writers::ACWritable for CommunicationChatRoomTracker {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationChatRoomTracker").entered();

        write_u32(writer, self.allegiance_room_id)?;
        write_u32(writer, self.general_chat_room_id)?;
        write_u32(writer, self.trade_chat_room_id)?;
        write_u32(writer, self.lfg_chat_room_id)?;
        write_u32(writer, self.roleplay_chat_room_id)?;
        write_u32(writer, self.olthoi_chat_room_id)?;
        write_u32(writer, self.society_chat_room_id)?;
        write_u32(writer, self.society_celestial_hand_chat_room_id)?;
        write_u32(writer, self.society_eldrich_web_chat_room_id)?;
        write_u32(writer, self.society_radiant_blood_chat_room_id)?;
        Ok(())
    }
}

