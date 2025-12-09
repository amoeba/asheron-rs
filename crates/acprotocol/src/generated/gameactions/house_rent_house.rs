use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Pay rent for a house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RentHouse")]
pub struct HouseRentHouse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ObjectId>,
}

impl HouseRentHouse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            object_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for HouseRentHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRentHouse::read(reader)
    }
}

