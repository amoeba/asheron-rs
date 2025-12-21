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

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendFriendsCommand")]
pub struct SocialSendFriendsCommand {
    #[serde(rename = "Command")]
    pub command: u32,
    #[serde(rename = "Player")]
    pub player: String,
}

impl crate::readers::ACDataType for SocialSendFriendsCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialSendFriendsCommand").entered();

        #[cfg(feature = "tracing")]
        let _field_span_command = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Command", position = pos).entered()
        };
        let command = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_command);
        #[cfg(feature = "tracing")]
        let _field_span_player = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Player", position = pos).entered()
        };
        let player = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_player);

        Ok(Self {
            command,
            player,
        })
    }
}

