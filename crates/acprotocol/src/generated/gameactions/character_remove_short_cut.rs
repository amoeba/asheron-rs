use serde::{Serialize, Deserialize};
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

impl crate::readers::ACDataType for CharacterRemoveShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let index = read_u32(reader)?;

        Ok(Self {
            index,
        })
    }
}

