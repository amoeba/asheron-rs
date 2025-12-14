use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Login of player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_CreatePlayer")]
pub struct LoginCreatePlayer {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
}

impl crate::readers::ACDataType for LoginCreatePlayer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;

        Ok(Self {
            character_id,
        })
    }
}

