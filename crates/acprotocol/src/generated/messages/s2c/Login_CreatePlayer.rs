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

// Login of player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_CreatePlayer")]
pub struct LoginCreatePlayer {
    #[serde(rename = "CharacterId")]
    pub character_id: ObjectId,
}

impl LoginCreatePlayer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let character_id = ObjectId::read(reader)?;

        Ok(Self {
            character_id,
        })
    }
}

impl crate::readers::ACDataType for LoginCreatePlayer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        LoginCreatePlayer::read(reader)
    }
}

