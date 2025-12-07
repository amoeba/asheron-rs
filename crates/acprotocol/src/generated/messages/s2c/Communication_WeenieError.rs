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

// Display a status message in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_WeenieError")]
pub struct CommunicationWeenieError {
    #[serde(rename = "Type")]
    pub type_: WeenieError,
}

impl CommunicationWeenieError {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            type_,
        })
    }
}

impl crate::readers::ACDataType for CommunicationWeenieError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationWeenieError::read(reader)
    }
}

