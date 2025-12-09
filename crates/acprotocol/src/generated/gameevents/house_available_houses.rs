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

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AvailableHouses")]
pub struct HouseAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
    #[serde(rename = "Houses")]
    pub houses: PackableList<uint>,
    #[serde(rename = "NumHouses")]
    pub num_houses: i32,
}

impl HouseAvailableHouses {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        let houses = read_packable_list::<u32>(reader)?;
        let num_houses = read_i32(reader)?;

        Ok(Self {
            type_,
            houses,
            num_houses,
        })
    }
}

impl crate::readers::ACDataType for HouseAvailableHouses {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAvailableHouses::read(reader)
    }
}

