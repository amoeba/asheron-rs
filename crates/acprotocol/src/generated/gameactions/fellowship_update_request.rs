use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateRequest")]
pub struct FellowshipUpdateRequest {
    #[serde(rename = "On")]
    pub on: bool,
}

impl crate::readers::ACDataType for FellowshipUpdateRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

