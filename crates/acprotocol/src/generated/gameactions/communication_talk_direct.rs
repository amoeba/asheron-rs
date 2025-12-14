use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Direct message by Id
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirect")]
pub struct CommunicationTalkDirect {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
}

impl crate::readers::ACDataType for CommunicationTalkDirect {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let target_id = ObjectId::read(reader)?;

        Ok(Self {
            message,
            target_id,
        })
    }
}

