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

// Update an item's mana bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_QueryItemManaResponse")]
pub struct ItemQueryItemManaResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Mana")]
    pub mana: f32,
    #[serde(rename = "Success")]
    pub success: bool,
}

impl ItemQueryItemManaResponse {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;
        let mana = read_f32(reader)?;
        let success = read_bool(reader)?;

        Ok(Self {
            object_id,
            mana,
            success,
        })
    }
}

impl crate::readers::ACDataType for ItemQueryItemManaResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemQueryItemManaResponse::read(reader)
    }
}

