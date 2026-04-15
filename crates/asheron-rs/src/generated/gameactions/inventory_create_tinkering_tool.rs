use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InventoryCreateTinkeringTool").entered();

        #[cfg(feature = "tracing")]
        let _field_span_tool_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ToolId", position = pos).entered()
        };
        let tool_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_tool_id);
        #[cfg(feature = "tracing")]
        let _field_span_items = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Items", position = pos).entered()
        };
        let items = read_packable_list::<ObjectId>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_items);

        Ok(Self {
            tool_id,
            items,
        })
    }
}

impl crate::writers::ACWritable for InventoryCreateTinkeringTool {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InventoryCreateTinkeringTool").entered();

        self.tool_id.write(writer)?;
        write_packable_list::<ObjectId>(writer, &self.items)?;
        Ok(())
    }
}

