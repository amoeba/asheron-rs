use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Failure to log in
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterError")]
pub struct CharacterCharacterError {
    #[serde(rename = "Reason")]
    pub reason: CharacterErrorType,
}

impl CharacterCharacterError {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let reason = CharacterErrorType::try_from(read_u32(reader)?)?;

        Ok(Self {
            reason,
        })
    }
}

impl crate::readers::ACDataType for CharacterCharacterError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterError::read(reader)
    }
}

