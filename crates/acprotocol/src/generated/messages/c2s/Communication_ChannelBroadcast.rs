use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

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

impl CommunicationChannelBroadcast {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;
        let sender_name = read_string(reader)?;
        let message = read_string(reader)?;

        Ok(Self {
            channel,
            sender_name,
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelBroadcast {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelBroadcast::read(reader)
    }
}

