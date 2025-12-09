use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Lists available house /house available
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ListAvailableHouses")]
pub struct HouseListAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
}

impl HouseListAvailableHouses {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = HouseType::try_from(read_u32(reader)?)?;

        Ok(Self {
            type_,
        })
    }
}

impl crate::readers::ACDataType for HouseListAvailableHouses {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseListAvailableHouses::read(reader)
    }
}

