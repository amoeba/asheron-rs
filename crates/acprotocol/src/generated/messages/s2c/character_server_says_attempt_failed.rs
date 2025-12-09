use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Failure to give an item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ServerSaysAttemptFailed")]
pub struct CharacterServerSaysAttemptFailed {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Reason")]
    pub reason: WeenieError,
}

impl CharacterServerSaysAttemptFailed {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let reason = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            object_id,
            reason,
        })
    }
}

impl crate::readers::ACDataType for CharacterServerSaysAttemptFailed {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterServerSaysAttemptFailed::read(reader)
    }
}

