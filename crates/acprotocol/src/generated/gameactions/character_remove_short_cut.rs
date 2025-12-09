use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Remove an item from the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveShortCut")]
pub struct CharacterRemoveShortCut {
    #[serde(rename = "Index")]
    pub index: u32,
}

impl CharacterRemoveShortCut {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let index = read_u32(reader)?;

        Ok(Self {
            index,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemoveShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemoveShortCut::read(reader)
    }
}

