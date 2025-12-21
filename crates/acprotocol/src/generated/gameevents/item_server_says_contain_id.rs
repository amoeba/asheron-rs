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

// Store an item in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysContainId")]
pub struct ItemServerSaysContainId {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
    #[serde(rename = "ContainerType")]
    pub container_type: ContainerProperties,
}

impl crate::readers::ACDataType for ItemServerSaysContainId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemServerSaysContainId").entered();

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
        let _field_span_container_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContainerType", position = pos).entered()
        };
        let container_type = ContainerProperties::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_container_type);

        Ok(Self {
            object_id,
            container_id,
            slot_index,
            container_type,
        })
    }
}

