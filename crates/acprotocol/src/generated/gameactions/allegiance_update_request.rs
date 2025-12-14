use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Allegiance update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_UpdateRequest")]
pub struct AllegianceUpdateRequest {
    #[serde(rename = "On")]
    pub on: bool,
}

impl crate::readers::ACDataType for AllegianceUpdateRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let on = read_bool(reader)?;

        Ok(Self {
            on,
        })
    }
}

