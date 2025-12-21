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

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ParentEvent")]
pub struct ItemParentEvent {
    #[serde(rename = "ParentId")]
    pub parent_id: ObjectId,
    #[serde(rename = "ChildId")]
    pub child_id: ObjectId,
    #[serde(rename = "Location")]
    pub location: ParentLocation,
    #[serde(rename = "Placement")]
    pub placement: Placement,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ChildPositionSequence")]
    pub child_position_sequence: u16,
}

impl crate::readers::ACDataType for ItemParentEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemParentEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_parent_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ParentId", position = pos).entered()
        };
        let parent_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_parent_id);
        #[cfg(feature = "tracing")]
        let _field_span_child_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ChildId", position = pos).entered()
        };
        let child_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_child_id);
        #[cfg(feature = "tracing")]
        let _field_span_location = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Location", position = pos).entered()
        };
        let location = ParentLocation::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_location);
        #[cfg(feature = "tracing")]
        let _field_span_placement = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Placement", position = pos).entered()
        };
        let placement = Placement::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_placement);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_child_position_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ChildPositionSequence", position = pos).entered()
        };
        let child_position_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_child_position_sequence);

        Ok(Self {
            parent_id,
            child_id,
            location,
            placement,
            object_instance_sequence,
            child_position_sequence,
        })
    }
}

