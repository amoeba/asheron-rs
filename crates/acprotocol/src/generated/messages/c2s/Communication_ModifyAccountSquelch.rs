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

