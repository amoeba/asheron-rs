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

// Emote message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_Emote")]
pub struct CommunicationEmote {
    #[serde(rename = "Message")]
    pub message: String,
}

impl crate::readers::ACDataType for CommunicationEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationEmote").entered();

        #[cfg(feature = "tracing")]
        let _field_span_message = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Message", position = pos).entered()
        };
        let message = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_message);

        Ok(Self {
            message,
        })
    }
}

