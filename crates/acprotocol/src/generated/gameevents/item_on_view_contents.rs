use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set Pack Contents
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_OnViewContents")]
pub struct ItemOnViewContents {
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ContentProfile>,
}

impl ItemOnViewContents {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let container_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ContentProfile>(reader)?;

        Ok(Self {
            container_id,
            items,
        })
    }
}

impl crate::readers::ACDataType for ItemOnViewContents {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        ItemOnViewContents::read(reader)
    }
}

