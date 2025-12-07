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

// For stackable items, this changes the number of items in the stack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateStackSize")]
pub struct ItemUpdateStackSize {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
    #[serde(rename = "NewValue")]
    pub new_value: u32,
}

impl ItemUpdateStackSize {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let amount = read_u32(reader)?;
        let new_value = read_u32(reader)?;

        Ok(Self {
            sequence,
            object_id,
            amount,
            new_value,
        })
    }
}

impl crate::readers::ACDataType for ItemUpdateStackSize {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemUpdateStackSize::read(reader)
    }
}

