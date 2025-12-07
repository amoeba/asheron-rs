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

