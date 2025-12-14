use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Display a status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieError")]
pub struct CommunicationWeenieError {
    #[serde(rename = "Type")]
    pub type_: WeenieError,
}

impl crate::readers::ACDataType for CommunicationWeenieError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            type_,
        })
    }
}

