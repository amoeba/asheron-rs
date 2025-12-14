use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Adds a friend
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddFriend")]
pub struct SocialAddFriend {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl crate::readers::ACDataType for SocialAddFriend {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_name = read_string(reader)?;

        Ok(Self {
            character_name,
        })
    }
}

