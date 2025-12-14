use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sent when picking up an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_PickupEvent")]
pub struct InventoryPickupEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectPositionSequence")]
    pub object_position_sequence: u16,
}

impl crate::readers::ACDataType for InventoryPickupEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let object_instance_sequence = read_u16(reader)?;
        let object_position_sequence = read_u16(reader)?;

        Ok(Self {
            object_id,
            object_instance_sequence,
            object_position_sequence,
        })
    }
}

