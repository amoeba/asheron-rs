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

