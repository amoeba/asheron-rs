use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyAccountSquelch")]
pub struct CommunicationModifyAccountSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl CommunicationModifyAccountSquelch {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;
        let character_name = read_string(reader)?;

        Ok(Self {
            add,
            character_name,
        })
    }
}

impl crate::readers::ACDataType for CommunicationModifyAccountSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationModifyAccountSquelch::read(reader)
    }
}

