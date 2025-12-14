use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// ChannelBroadcast: Group Chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelBroadcast")]
pub struct CommunicationChannelBroadcast {
    #[serde(rename = "Channel")]
    pub channel: Channel,
    #[serde(rename = "Message")]
    pub message: String,
}

impl crate::readers::ACDataType for CommunicationChannelBroadcast {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Ok::<_, Box<dyn std::error::Error>>(Channel::from_bits_retain(read_u32(reader)?))?;
        let message = read_string(reader)?;

        Ok(Self {
            channel,
            message,
        })
    }
}

