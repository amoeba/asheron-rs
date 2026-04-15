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

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_DeleteObject")]
pub struct ItemDeleteObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
}

impl crate::readers::ACDataType for ItemDeleteObject {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemDeleteObject").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

        Ok(Self {
            object_id,
            object_instance_sequence,
        })
    }
}

impl crate::writers::ACWritable for ItemDeleteObject {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemDeleteObject").entered();

        self.object_id.write(writer)?;
        write_u16(writer, self.object_instance_sequence)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

