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

// Direct message by Id
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_TalkDirect")]
pub struct CommunicationTalkDirect {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
}

impl CommunicationTalkDirect {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let message = read_string(reader)?;
        let target_id = ObjectId::read(reader)?;

        Ok(Self {
            message,
            target_id,
        })
    }
}

impl crate::readers::ACDataType for CommunicationTalkDirect {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTalkDirect::read(reader)
    }
}

