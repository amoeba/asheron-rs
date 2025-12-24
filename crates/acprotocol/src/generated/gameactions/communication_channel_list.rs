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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelList")]
pub struct CommunicationChannelList {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

impl crate::readers::ACDataType for CommunicationChannelList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationChannelList").entered();

        #[cfg(feature = "tracing")]
        let _field_span_channel = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Channel", position = pos).entered()
        };
        let channel = Ok::<_, Box<dyn std::error::Error>>(Channel::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_channel);

        Ok(Self {
            channel,
        })
    }
}

impl crate::writers::ACWritable for CommunicationChannelList {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationChannelList").entered();

        write_u32(writer, self.channel.bits())?;
        Ok(())
    }
}

