use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Boots a player from the allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatBoot")]
pub struct AllegianceAllegianceChatBoot {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Reason")]
    pub reason: String,
}

impl crate::readers::ACDataType for AllegianceAllegianceChatBoot {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;
        let reason = read_string(reader)?;

        Ok(Self {
            character_name,
            reason,
        })
    }
}

