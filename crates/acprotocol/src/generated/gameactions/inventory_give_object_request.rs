use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl crate::readers::ACDataType for InventoryGiveObjectRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

