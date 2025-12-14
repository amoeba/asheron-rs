use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Merges one stack with another
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableMerge")]
pub struct InventoryStackableMerge {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl crate::readers::ACDataType for InventoryStackableMerge {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let target_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            object_id,
            target_id,
            amount,
        })
    }
}

