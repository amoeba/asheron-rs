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

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearRangedSpeech")]
pub struct CommunicationHearRangedSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Range")]
    pub range: f32,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
}

impl crate::readers::ACDataType for CommunicationHearRangedSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationHearRangedSpeech").entered();

        #[cfg(feature = "tracing")]
        let _field_span_message = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Message", position = pos).entered()
        };
        let message = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_message);
        #[cfg(feature = "tracing")]
        let _field_span_sender_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SenderName", position = pos).entered()
        };
        let sender_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sender_name);
        #[cfg(feature = "tracing")]
        let _field_span_sender_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SenderId", position = pos).entered()
        };
        let sender_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sender_id);
        #[cfg(feature = "tracing")]
        let _field_span_range = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Range", position = pos).entered()
        };
        let range = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_range);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);

        Ok(Self {
            message,
            sender_name,
            sender_id,
            range,
            type_,
        })
    }
}

