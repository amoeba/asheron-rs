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

