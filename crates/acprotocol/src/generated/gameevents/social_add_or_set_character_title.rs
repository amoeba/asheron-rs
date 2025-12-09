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

// Set a title for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddOrSetCharacterTitle")]
pub struct SocialAddOrSetCharacterTitle {
    #[serde(rename = "NewTitle")]
    pub new_title: u32,
    #[serde(rename = "SetAsDisplayTitle")]
    pub set_as_display_title: bool,
}

impl SocialAddOrSetCharacterTitle {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let new_title = read_u32(reader)?;
        let set_as_display_title = read_bool(reader)?;

        Ok(Self {
            new_title,
            set_as_display_title,
        })
    }
}

impl crate::readers::ACDataType for SocialAddOrSetCharacterTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialAddOrSetCharacterTitle::read(reader)
    }
}

