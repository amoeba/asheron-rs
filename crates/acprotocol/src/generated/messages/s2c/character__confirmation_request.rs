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

// Display a confirmation panel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationRequest")]
pub struct CharacterConfirmationRequest {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

impl CharacterConfirmationRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let confirmation_type = ConfirmationType::try_from(read_u32(reader)?)?;
        let context_id = read_u32(reader)?;
        let text = read_string(reader)?;

        Ok(Self {
            confirmation_type,
            context_id,
            text,
        })
    }
}

impl crate::readers::ACDataType for CharacterConfirmationRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CharacterConfirmationRequest::read(reader)
    }
}

