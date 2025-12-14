use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Equip an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_WearItem")]
pub struct ItemWearItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
}

impl crate::readers::ACDataType for ItemWearItem {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let slot = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            object_id,
            slot,
        })
    }
}

