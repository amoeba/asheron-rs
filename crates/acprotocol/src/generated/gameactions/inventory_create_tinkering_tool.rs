use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Salvages items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_CreateTinkeringTool")]
pub struct InventoryCreateTinkeringTool {
    #[serde(rename = "ToolId")]
    pub tool_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ObjectId>,
}

impl crate::readers::ACDataType for InventoryCreateTinkeringTool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let tool_id = ObjectId::read(reader)?;
        let items = read_packable_list::<ObjectId>(reader)?;

        Ok(Self {
            tool_id,
            items,
        })
    }
}

