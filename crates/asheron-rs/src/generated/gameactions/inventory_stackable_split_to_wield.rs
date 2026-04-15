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

// Splits an item to a wield location.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitToWield")]
pub struct InventoryStackableSplitToWield {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Slot")]
    pub slot: EquipMask,
    #[serde(rename = "Amount")]
    pub amount: i32,
}

impl crate::readers::ACDataType for InventoryStackableSplitToWield {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "InventoryStackableSplitToWield").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_slot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Slot", position = pos).entered()
        };
        let slot = Ok::<_, Box<dyn std::error::Error>>(EquipMask::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_slot);
        #[cfg(feature = "tracing")]
        let _field_span_amount = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Amount", position = pos).entered()
        };
        let amount = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_amount);

        Ok(Self {
            object_id,
            slot,
            amount,
        })
    }
}

impl crate::writers::ACWritable for InventoryStackableSplitToWield {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "InventoryStackableSplitToWield").entered();

        self.object_id.write(writer)?;
        write_u32(writer, self.slot.bits())?;
        write_i32(writer, self.amount)?;
        Ok(())
    }
}

