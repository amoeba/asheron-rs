use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Update Rent Time
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentTime")]
pub struct HouseUpdateRentTime {
    #[serde(rename = "RentTime")]
    pub rent_time: u32,
}

impl crate::readers::ACDataType for HouseUpdateRentTime {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rent_time = read_u32(reader)?;

        Ok(Self {
            rent_time,
        })
    }
}

