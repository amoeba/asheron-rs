use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// House panel information for owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseData")]
pub struct HouseHouseData {
    #[serde(rename = "Data")]
    pub data: HouseData,
}

impl crate::readers::ACDataType for HouseHouseData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let data = HouseData::read(reader)?;

        Ok(Self {
            data,
        })
    }
}

