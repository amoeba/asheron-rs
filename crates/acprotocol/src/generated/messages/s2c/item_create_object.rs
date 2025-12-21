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

// Create an object somewhere in the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_CreateObject")]
pub struct ItemCreateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "PhysicsDescription")]
    pub physics_description: PhysicsDesc,
    #[serde(rename = "WeenieDescription")]
    pub weenie_description: PublicWeenieDesc,
}

impl crate::readers::ACDataType for ItemCreateObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemCreateObject").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_object_description = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectDescription", position = pos).entered()
        };
        let object_description = ObjDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_description);
        #[cfg(feature = "tracing")]
        let _field_span_physics_description = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PhysicsDescription", position = pos).entered()
        };
        let physics_description = PhysicsDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_physics_description);
        #[cfg(feature = "tracing")]
        let _field_span_weenie_description = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieDescription", position = pos).entered()
        };
        let weenie_description = PublicWeenieDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_description);

        Ok(Self {
            object_id,
            object_description,
            physics_description,
            weenie_description,
        })
    }
}

