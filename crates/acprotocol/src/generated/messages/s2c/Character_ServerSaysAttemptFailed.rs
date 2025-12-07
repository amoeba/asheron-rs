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

