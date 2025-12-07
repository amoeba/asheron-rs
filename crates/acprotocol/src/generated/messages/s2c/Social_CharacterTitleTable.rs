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

// Titles for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_CharacterTitleTable")]
pub struct SocialCharacterTitleTable {
    #[serde(rename = "DisplayTitle")]
    pub display_title: u32,
    #[serde(rename = "Titles")]
    pub titles: PackableList<uint>,
}

impl SocialCharacterTitleTable {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let display_title = read_u32(reader)?;
        let titles = read_packable_list::<u32>(reader)?;

        Ok(Self {
            display_title,
            titles,
        })
    }
}

impl crate::readers::ACDataType for SocialCharacterTitleTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialCharacterTitleTable::read(reader)
    }
}

