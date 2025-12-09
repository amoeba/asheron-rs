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

// Someone has sent you a @tell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_HearDirectSpeech")]
pub struct CommunicationHearDirectSpeech {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "SenderName")]
    pub sender_name: String,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: ChatFragmentType,
    #[serde(rename = "SecretFlags")]
    pub secret_flags: u32,
}

impl CommunicationHearDirectSpeech {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let sender_name = read_string(reader)?;
        let sender_id = ObjectId::read(reader)?;
        let target_id = ObjectId::read(reader)?;
        let type_ = ChatFragmentType::try_from(read_u32(reader)?)?;
        let secret_flags = read_u32(reader)?;

        Ok(Self {
            message,
            sender_name,
            sender_id,
            target_id,
            type_,
            secret_flags,
        })
    }
}

impl crate::readers::ACDataType for CommunicationHearDirectSpeech {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationHearDirectSpeech::read(reader)
    }
}

