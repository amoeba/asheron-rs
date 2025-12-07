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

// Emote message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_Emote")]
pub struct CommunicationEmote {
    #[serde(rename = "Message")]
    pub message: String,
}

impl CommunicationEmote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;

        Ok(Self {
            message,
        })
    }
}

impl crate::readers::ACDataType for CommunicationEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationEmote::read(reader)
    }
}

