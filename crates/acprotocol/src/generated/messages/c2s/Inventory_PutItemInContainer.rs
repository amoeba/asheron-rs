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

// Store an item in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_PutItemInContainer")]
pub struct InventoryPutItemInContainer {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
}

impl InventoryPutItemInContainer {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let container_id = ObjectId::read(reader)?;
        let slot_index = read_u32(reader)?;

        Ok(Self {
            object_id,
            container_id,
            slot_index,
        })
    }
}

impl crate::readers::ACDataType for InventoryPutItemInContainer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        InventoryPutItemInContainer::read(reader)
    }
}

