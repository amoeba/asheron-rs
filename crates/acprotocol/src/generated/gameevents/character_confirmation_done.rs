use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Confirmation done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationDone")]
pub struct CharacterConfirmationDone {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
}

impl CharacterConfirmationDone {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let confirmation_type = ConfirmationType::try_from(read_u32(reader)?)?;
        let context_id = read_u32(reader)?;

        Ok(Self {
            confirmation_type,
            context_id,
        })
    }
}

impl crate::readers::ACDataType for CharacterConfirmationDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterConfirmationDone::read(reader)
    }
}

