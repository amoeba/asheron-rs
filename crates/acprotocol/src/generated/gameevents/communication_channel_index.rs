use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelIndex")]
pub struct CommunicationChannelIndex {
    #[serde(rename = "Channels")]
    pub channels: PackableList<String>,
}

impl crate::readers::ACDataType for CommunicationChannelIndex {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channels = read_packable_list::<String>(reader)?;

        Ok(Self {
            channels,
        })
    }
}

