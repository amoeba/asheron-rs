use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// The list of characters on the current account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_LoginCharacterSet")]
pub struct LoginLoginCharacterSet {
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Characters")]
    pub characters: PackableList<CharacterIdentity>,
    #[serde(rename = "DeletedCharacters")]
    pub deleted_characters: PackableList<CharacterIdentity>,
    #[serde(rename = "NumAllowedCharacters")]
    pub num_allowed_characters: u32,
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "UseTurbineChat")]
    pub use_turbine_chat: bool,
    #[serde(rename = "HasThroneofDestiny")]
    pub has_throneof_destiny: bool,
}

impl crate::readers::ACDataType for LoginLoginCharacterSet {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "LoginLoginCharacterSet").entered();

        #[cfg(feature = "tracing")]
        let _field_span_status = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Status", position = pos).entered()
        };
        let status = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_status);
        #[cfg(feature = "tracing")]
        let _field_span_characters = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Characters", position = pos).entered()
        };
        let characters = read_packable_list::<CharacterIdentity>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_characters);
        #[cfg(feature = "tracing")]
        let _field_span_deleted_characters = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DeletedCharacters", position = pos).entered()
        };
        let deleted_characters = read_packable_list::<CharacterIdentity>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_deleted_characters);
        #[cfg(feature = "tracing")]
        let _field_span_num_allowed_characters = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NumAllowedCharacters", position = pos).entered()
        };
        let num_allowed_characters = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_num_allowed_characters);
        #[cfg(feature = "tracing")]
        let _field_span_account = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Account", position = pos).entered()
        };
        let account = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account);
        #[cfg(feature = "tracing")]
        let _field_span_use_turbine_chat = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "UseTurbineChat", position = pos).entered()
        };
        let use_turbine_chat = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_use_turbine_chat);
        #[cfg(feature = "tracing")]
        let _field_span_has_throneof_destiny = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HasThroneofDestiny", position = pos).entered()
        };
        let has_throneof_destiny = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_has_throneof_destiny);

        Ok(Self {
            status,
            characters,
            deleted_characters,
            num_allowed_characters,
            account,
            use_turbine_chat,
            has_throneof_destiny,
        })
    }
}

