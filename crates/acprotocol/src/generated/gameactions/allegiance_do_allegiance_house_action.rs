use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Perform the allegiance house action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_DoAllegianceHouseAction")]
pub struct AllegianceDoAllegianceHouseAction {
    #[serde(rename = "Action")]
    pub action: AllegianceHouseAction,
}

impl AllegianceDoAllegianceHouseAction {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let action = AllegianceHouseAction::try_from(read_u32(reader)?)?;

        Ok(Self {
            action,
        })
    }
}

impl crate::readers::ACDataType for AllegianceDoAllegianceHouseAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        AllegianceDoAllegianceHouseAction::read(reader)
    }
}

