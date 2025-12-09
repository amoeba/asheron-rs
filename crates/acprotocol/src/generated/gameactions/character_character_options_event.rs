use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set multiple character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterOptionsEvent")]
pub struct CharacterCharacterOptionsEvent {
    #[serde(rename = "Options")]
    pub options: PlayerModule,
}

impl CharacterCharacterOptionsEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let options = PlayerModule::read(reader)?;

        Ok(Self {
            options,
        })
    }
}

impl crate::readers::ACDataType for CharacterCharacterOptionsEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterOptionsEvent::read(reader)
    }
}

