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

// Mark a character for deletetion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterDelete")]
pub struct CharacterCharacterDelete {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Slot")]
    pub slot: i32,
}

impl CharacterCharacterDelete {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account = read_string(reader)?;
        let slot = read_i32(reader)?;

        Ok(Self {
            account,
            slot,
        })
    }
}

impl crate::readers::ACDataType for CharacterCharacterDelete {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterCharacterDelete::read(reader)
    }
}

