use serde::{Serialize, Deserialize};
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

impl crate::readers::ACDataType for HouseSetOpenHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open_house = read_bool(reader)?;

        Ok(Self {
            open_house,
        })
    }
}

