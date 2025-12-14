use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Query your house info, during signin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_QueryHouse")]
pub struct HouseQueryHouse {}

impl HouseQueryHouse {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseQueryHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseQueryHouse::read(reader)
    }
}

