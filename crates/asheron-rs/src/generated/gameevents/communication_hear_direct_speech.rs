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

// Someone has sent you a @tell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearDirectSpeech")]
pub struct CommunicationHearDirectSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
    #[serde(rename = "SecretFlags")]
    pub secret_flags: u32,
}

impl crate::readers::ACDataType for CommunicationHearDirectSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationHearDirectSpeech").entered();

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
        let _field_span_target_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TargetId", position = pos).entered()
        };
        let target_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_target_id);
        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_secret_flags = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SecretFlags", position = pos).entered()
        };
        let secret_flags = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_secret_flags);

        Ok(Self {
            message,
            sender_name,
            sender_id,
            target_id,
            type_,
            secret_flags,
        })
    }
}

impl crate::writers::ACWritable for CommunicationHearDirectSpeech {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationHearDirectSpeech").entered();

        write_string(writer, &self.message)?;
        write_string(writer, &self.sender_name)?;
        self.sender_id.write(writer)?;
        self.target_id.write(writer)?;
        write_u32(writer, self.type_.clone() as u32)?;
        write_u32(writer, self.secret_flags)?;
        Ok(())
    }
}

