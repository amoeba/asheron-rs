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

// Confirms a dialog
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationResponse")]
pub struct CharacterConfirmationResponse {
    #[serde(rename = "Type")]
    pub type_: ConfirmationType,
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Accepted")]
    pub accepted: bool,
}

impl CharacterConfirmationResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = ConfirmationType::try_from(read_u32(reader)?)?;
        let context = read_u32(reader)?;
        let accepted = read_bool(reader)?;

        Ok(Self {
            type_,
            context,
            accepted,
        })
    }
}

impl crate::readers::ACDataType for CharacterConfirmationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterConfirmationResponse::read(reader)
    }
}

