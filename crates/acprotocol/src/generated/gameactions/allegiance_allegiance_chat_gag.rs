use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Gags a person in allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatGag")]
pub struct AllegianceAllegianceChatGag {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "On")]
    pub on: bool,
}

impl crate::readers::ACDataType for AllegianceAllegianceChatGag {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;
        let on = read_bool(reader)?;

        Ok(Self {
            character_name,
            on,
        })
    }
}

