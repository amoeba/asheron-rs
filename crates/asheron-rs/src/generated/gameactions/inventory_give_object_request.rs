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

// Give an item to someone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_GiveObjectRequest")]
pub struct InventoryGiveObjectRequest {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl crate::readers::ACDataType for InventoryGiveObjectRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InventoryGiveObjectRequest").entered();

        #[cfg(feature = "tracing")]
        let _field_span_target_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TargetId", position = pos).entered()
        };
        let target_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_target_id);
        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_amount = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Amount", position = pos).entered()
        };
        let amount = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_amount);

        Ok(Self {
            target_id,
            object_id,
            amount,
        })
    }
}

impl crate::writers::ACWritable for InventoryGiveObjectRequest {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InventoryGiveObjectRequest").entered();

        self.target_id.write(writer)?;
        self.object_id.write(writer)?;
        write_u32(writer, self.amount)?;
        Ok(())
    }
}

