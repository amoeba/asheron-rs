use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Character creation result
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SendCharGenResult")]
pub struct CharacterSendCharGenResult {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Result")]
    pub result: CharGenResult,
}

impl CharacterSendCharGenResult {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let account = read_string(reader)?;
        let result = CharGenResult::read(reader)?;

        Ok(Self {
            account,
            result,
        })
    }
}

impl crate::readers::ACDataType for CharacterSendCharGenResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterSendCharGenResult::read(reader)
    }
}

