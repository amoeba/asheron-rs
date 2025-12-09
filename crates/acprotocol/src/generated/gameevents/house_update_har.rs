use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// House Guest List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateHAR")]
pub struct HouseUpdateHAR {
    #[serde(rename = "GuestList")]
    pub guest_list: HAR,
}

impl HouseUpdateHAR {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_list = HAR::read(reader)?;

        Ok(Self {
            guest_list,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateHAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateHAR::read(reader)
    }
}

