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

// Indirect '/e' text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearEmote")]
pub struct CommunicationHearEmote {
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Text")]
    pub text: String,
}

impl crate::readers::ACDataType for CommunicationHearEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationHearEmote").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sender_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SenderId", position = pos).entered()
        };
        let sender_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sender_id);
        #[cfg(feature = "tracing")]
        let _field_span_sender_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SenderName", position = pos).entered()
        };
        let sender_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sender_name);
        #[cfg(feature = "tracing")]
        let _field_span_text = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Text", position = pos).entered()
        };
        let text = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_text);

        Ok(Self {
            sender_id,
            sender_name,
            text,
        })
    }
}

impl crate::writers::ACWritable for CommunicationHearEmote {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationHearEmote").entered();

        self.sender_id.write(writer)?;
        write_string(writer, &self.sender_name)?;
        write_string(writer, &self.text)?;
        Ok(())
    }
}

