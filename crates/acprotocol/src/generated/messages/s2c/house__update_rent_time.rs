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

// Update Rent Time
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentTime")]
pub struct HouseUpdateRentTime {
    #[serde(rename = "RentTime")]
    pub rent_time: u32,
}

impl HouseUpdateRentTime {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rent_time = read_u32(reader)?;

        Ok(Self {
            rent_time,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateRentTime {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateRentTime::read(reader)
    }
}

