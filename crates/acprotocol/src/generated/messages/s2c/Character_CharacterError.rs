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

