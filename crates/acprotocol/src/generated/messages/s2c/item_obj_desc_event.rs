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

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ObjDescEvent")]
pub struct ItemObjDescEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "InstanceSequence")]
    pub instance_sequence: u16,
    #[serde(rename = "VisualDescSequence")]
    pub visual_desc_sequence: u16,
}

impl crate::readers::ACDataType for ItemObjDescEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemObjDescEvent").entered();

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
        let _field_span_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "InstanceSequence", position = pos).entered()
        };
        let instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_visual_desc_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "VisualDescSequence", position = pos).entered()
        };
        let visual_desc_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_visual_desc_sequence);

        Ok(Self {
            object_id,
            object_description,
            instance_sequence,
            visual_desc_sequence,
        })
    }
}

impl crate::writers::ACWritable for ItemObjDescEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemObjDescEvent").entered();

        self.object_id.write(writer)?;
        self.object_description.write(writer)?;
        write_u16(writer, self.instance_sequence)?;
        write_u16(writer, self.visual_desc_sequence)?;
        Ok(())
    }
}

