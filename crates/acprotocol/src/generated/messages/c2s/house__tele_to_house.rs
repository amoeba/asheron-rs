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

// Teleports you to your house, /house recall
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_TeleToHouse")]
pub struct HouseTeleToHouse {}

impl HouseTeleToHouse {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseTeleToHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseTeleToHouse::read(reader)
    }
}

