use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Leaves a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_RemoveFromChannel")]
pub struct CommunicationRemoveFromChannel {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

impl crate::readers::ACDataType for CommunicationRemoveFromChannel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Ok::<_, Box<dyn std::error::Error>>(Channel::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            channel,
        })
    }
}

