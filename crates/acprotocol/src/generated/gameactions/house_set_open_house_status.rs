use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets your house open status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetOpenHouseStatus")]
pub struct HouseSetOpenHouseStatus {
    #[serde(rename = "OpenHouse")]
    pub open_house: bool,
}

impl HouseSetOpenHouseStatus {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open_house = read_bool(reader)?;

        Ok(Self {
            open_house,
        })
    }
}

impl crate::readers::ACDataType for HouseSetOpenHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseSetOpenHouseStatus::read(reader)
    }
}

