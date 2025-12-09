use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Revokes a player's corpse looting permission, /permit remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemovePlayerPermission")]
pub struct CharacterRemovePlayerPermission {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl CharacterRemovePlayerPermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

impl crate::readers::ACDataType for CharacterRemovePlayerPermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterRemovePlayerPermission::read(reader)
    }
}

