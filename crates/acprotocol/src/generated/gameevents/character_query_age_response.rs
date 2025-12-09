use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_QueryAgeResponse")]
pub struct CharacterQueryAgeResponse {
    #[serde(rename = "TargetName")]
    pub target_name: String,
    #[serde(rename = "Age")]
    pub age: String,
}

impl CharacterQueryAgeResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;
        let age = read_string(reader)?;

        Ok(Self {
            target_name,
            age,
        })
    }
}

impl crate::readers::ACDataType for CharacterQueryAgeResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterQueryAgeResponse::read(reader)
    }
}

