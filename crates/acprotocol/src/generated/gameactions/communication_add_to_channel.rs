use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Joins a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_AddToChannel")]
pub struct CommunicationAddToChannel {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

impl CommunicationAddToChannel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Ok::<_, Box<dyn std::error::Error>>(Channel::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            channel,
        })
    }
}

impl crate::readers::ACDataType for CommunicationAddToChannel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationAddToChannel::read(reader)
    }
}

