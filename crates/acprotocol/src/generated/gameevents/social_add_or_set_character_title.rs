use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set a title for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddOrSetCharacterTitle")]
pub struct SocialAddOrSetCharacterTitle {
    #[serde(rename = "NewTitle")]
    pub new_title: u32,
    #[serde(rename = "SetAsDisplayTitle")]
    pub set_as_display_title: bool,
}

impl crate::readers::ACDataType for SocialAddOrSetCharacterTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let new_title = read_u32(reader)?;
        let set_as_display_title = read_bool(reader)?;

        Ok(Self {
            new_title,
            set_as_display_title,
        })
    }
}

