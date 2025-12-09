use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Split a stack and place it into the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitTo3D")]
pub struct InventoryStackableSplitTo3D {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl InventoryStackableSplitTo3D {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            object_id,
            amount,
        })
    }
}

impl crate::readers::ACDataType for InventoryStackableSplitTo3D {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryStackableSplitTo3D::read(reader)
    }
}

