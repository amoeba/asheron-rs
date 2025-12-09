use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Add an item to the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddShortCut")]
pub struct CharacterAddShortCut {
    #[serde(rename = "Shortcut")]
    pub shortcut: ShortCutData,
}

impl CharacterAddShortCut {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let shortcut = ShortCutData::read(reader)?;

        Ok(Self {
            shortcut,
        })
    }
}

impl crate::readers::ACDataType for CharacterAddShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterAddShortCut::read(reader)
    }
}

