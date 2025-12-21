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

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelIndex")]
pub struct CommunicationChannelIndex {
    #[serde(rename = "Channels")]
    pub channels: PackableList<String>,
}

impl crate::readers::ACDataType for CommunicationChannelIndex {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationChannelIndex").entered();

        #[cfg(feature = "tracing")]
        let _field_span_channels = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Channels", position = pos).entered()
        };
        let channels = read_packable_list::<String>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_channels);

        Ok(Self {
            channels,
        })
    }
}

