use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl crate::readers::ACDataType for CharacterConfirmationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

