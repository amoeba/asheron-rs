use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set AFK mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetAFKMode")]
pub struct CommunicationSetAFKMode {
    #[serde(rename = "AFK")]
    pub afk: bool,
}

impl crate::readers::ACDataType for CommunicationSetAFKMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let afk = read_bool(reader)?;

        Ok(Self {
            afk,
        })
    }
}

