use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets a character's display title
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SetDisplayCharacterTitle")]
pub struct SocialSetDisplayCharacterTitle {
    #[serde(rename = "TitleId")]
    pub title_id: u32,
}

impl SocialSetDisplayCharacterTitle {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let title_id = read_u32(reader)?;

        Ok(Self {
            title_id,
        })
    }
}

impl crate::readers::ACDataType for SocialSetDisplayCharacterTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        SocialSetDisplayCharacterTitle::read(reader)
    }
}

