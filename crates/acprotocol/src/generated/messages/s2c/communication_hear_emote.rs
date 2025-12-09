use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Indirect '/e' text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearEmote")]
pub struct CommunicationHearEmote {
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Text")]
    pub text: String,
}

impl CommunicationHearEmote {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sender_id = ObjectId::read(reader)?;
        let sender_name = read_string(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            sender_id,
            sender_name,
            text,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearEmote::read(reader)
    }
}

