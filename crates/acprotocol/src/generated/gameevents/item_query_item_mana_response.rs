use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

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

impl crate::readers::ACDataType for ItemQueryItemManaResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemQueryItemManaResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_mana = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Mana", position = pos).entered()
        };
        let mana = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mana);
        #[cfg(feature = "tracing")]
        let _field_span_success = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Success", position = pos).entered()
        };
        let success = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_success);

        Ok(Self {
            object_id,
            mana,
            success,
        })
    }
}

