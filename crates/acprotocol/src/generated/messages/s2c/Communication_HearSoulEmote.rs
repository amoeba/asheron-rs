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

// Contains the text associated with an emote action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearSoulEmote")]
pub struct CommunicationHearSoulEmote {
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "Text")]
    pub text: String,
}

impl CommunicationHearSoulEmote {
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

impl crate::readers::ACDataType for CommunicationHearSoulEmote {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearSoulEmote::read(reader)
    }
}

