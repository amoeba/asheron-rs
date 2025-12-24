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

// Split a stack and place it into a container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitToContainer")]
pub struct InventoryStackableSplitToContainer {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl crate::readers::ACDataType for InventoryStackableSplitToContainer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InventoryStackableSplitToContainer").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_container_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContainerId", position = pos).entered()
        };
        let container_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_container_id);
        #[cfg(feature = "tracing")]
        let _field_span_slot_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SlotIndex", position = pos).entered()
        };
        let slot_index = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot_index);
        #[cfg(feature = "tracing")]
        let _field_span_amount = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Amount", position = pos).entered()
        };
        let amount = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_amount);

        Ok(Self {
            object_id,
            container_id,
            slot_index,
            amount,
        })
    }
}

impl crate::writers::ACWritable for InventoryStackableSplitToContainer {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InventoryStackableSplitToContainer").entered();

        self.object_id.write(writer)?;
        self.container_id.write(writer)?;
        write_u32(writer, self.slot_index)?;
        write_u32(writer, self.amount)?;
        Ok(())
    }
}

