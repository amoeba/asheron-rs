use serde::{Serialize, Deserialize};
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

impl crate::readers::ACDataType for AllegianceDoAllegianceHouseAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let action = AllegianceHouseAction::try_from(read_u32(reader)?)?;

        Ok(Self {
            action,
        })
    }
}

