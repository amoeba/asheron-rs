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

// Update an existing object's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateObject")]
pub struct ItemUpdateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDesc")]
    pub object_desc: ObjDesc,
    #[serde(rename = "PhysicsDesc")]
    pub physics_desc: PhysicsDesc,
    #[serde(rename = "WeenieDesc")]
    pub weenie_desc: PublicWeenieDesc,
}

impl crate::readers::ACDataType for ItemUpdateObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemUpdateObject").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_object_desc = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectDesc", position = pos).entered()
        };
        let object_desc = ObjDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_desc);
        #[cfg(feature = "tracing")]
        let _field_span_physics_desc = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "PhysicsDesc", position = pos).entered()
        };
        let physics_desc = PhysicsDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_physics_desc);
        #[cfg(feature = "tracing")]
        let _field_span_weenie_desc = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "WeenieDesc", position = pos).entered()
        };
        let weenie_desc = PublicWeenieDesc::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_weenie_desc);

        Ok(Self {
            object_id,
            object_desc,
            physics_desc,
            weenie_desc,
        })
    }
}

impl crate::writers::ACWritable for ItemUpdateObject {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemUpdateObject").entered();

        self.object_id.write(writer)?;
        self.object_desc.write(writer)?;
        self.physics_desc.write(writer)?;
        self.weenie_desc.write(writer)?;
        Ok(())
    }
}

