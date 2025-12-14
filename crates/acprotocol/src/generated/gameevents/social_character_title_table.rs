use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Titles for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_CharacterTitleTable")]
pub struct SocialCharacterTitleTable {
    #[serde(rename = "DisplayTitle")]
    pub display_title: u32,
    #[serde(rename = "Titles")]
    pub titles: PackableList<u32>,
}

impl crate::readers::ACDataType for SocialCharacterTitleTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let display_title = read_u32(reader)?;
        let titles = read_packable_list::<u32>(reader)?;

        Ok(Self {
            display_title,
            titles,
        })
    }
}

