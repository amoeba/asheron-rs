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

// Attempt to use an item with a target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_UseWithTargetEvent")]
pub struct InventoryUseWithTargetEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
}

impl InventoryUseWithTargetEvent {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let target_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
            target_id,
        })
    }
}

impl crate::readers::ACDataType for InventoryUseWithTargetEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryUseWithTargetEvent::read(reader)
    }
}

