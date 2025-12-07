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

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysMoveItem")]
pub struct ItemServerSaysMoveItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

impl ItemServerSaysMoveItem {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let object_id = ObjectId::read(reader)?;

        Ok(Self {
            object_id,
        })
    }
}

impl crate::readers::ACDataType for ItemServerSaysMoveItem {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemServerSaysMoveItem::read(reader)
    }
}

