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

// Sends a message to a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelBroadcast")]
pub struct CommunicationChannelBroadcast {
    #[serde(rename = "Channel")]
    pub channel: Channel,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Message")]
    pub message: String,
}

impl crate::readers::ACDataType for CommunicationChannelBroadcast {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationChannelBroadcast").entered();

        #[cfg(feature = "tracing")]
        let _field_span_channel = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Channel", position = pos).entered()
        };
        let channel = Ok::<_, Box<dyn std::error::Error>>(Channel::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_channel);
        #[cfg(feature = "tracing")]
        let _field_span_sender_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SenderName", position = pos).entered()
        };
        let sender_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sender_name);
        #[cfg(feature = "tracing")]
        let _field_span_message = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Message", position = pos).entered()
        };
        let message = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_message);

        Ok(Self {
            channel,
            sender_name,
            message,
        })
    }
}

impl crate::writers::ACWritable for CommunicationChannelBroadcast {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationChannelBroadcast").entered();

        write_u32(writer, self.channel.bits())?;
        write_string(writer, &self.sender_name)?;
        write_string(writer, &self.message)?;
        Ok(())
    }
}

