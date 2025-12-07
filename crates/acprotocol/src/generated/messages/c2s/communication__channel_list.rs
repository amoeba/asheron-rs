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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChannelList")]
pub struct CommunicationChannelList {
    #[serde(rename = "Channel")]
    pub channel: Channel,
}

impl CommunicationChannelList {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::try_from(read_u32(reader)?)?;

        Ok(Self {
            channel,
        })
    }
}

impl crate::readers::ACDataType for CommunicationChannelList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationChannelList::read(reader)
    }
}

