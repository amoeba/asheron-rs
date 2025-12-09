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

// Give an item to someone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_GiveObjectRequest")]
pub struct InventoryGiveObjectRequest {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl InventoryGiveObjectRequest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_id = ObjectId::read(reader)?;
        let object_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            target_id,
            object_id,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryGiveObjectRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryGiveObjectRequest::read(reader)
    }
}

