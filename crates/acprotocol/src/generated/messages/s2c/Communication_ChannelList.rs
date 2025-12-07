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

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelList")]
pub struct CommunicationChannelList {
    #[serde(rename = "Characters")]
    pub characters: PackableList<string>,
}

impl CommunicationChannelList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let characters = read_packable_list::<String>(reader)?;

        Ok(Self {
            characters,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelList::read(reader)
    }
}

