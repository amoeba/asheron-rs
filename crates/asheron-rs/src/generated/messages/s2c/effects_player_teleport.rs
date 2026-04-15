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

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayerTeleport")]
pub struct EffectsPlayerTeleport {
    #[serde(rename = "ObjectTeleportSequence")]
    pub object_teleport_sequence: u16,
}

impl crate::readers::ACDataType for EffectsPlayerTeleport {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "EffectsPlayerTeleport").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_teleport_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectTeleportSequence", position = pos).entered()
        };
        let object_teleport_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_teleport_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

        Ok(Self {
            object_teleport_sequence,
        })
    }
}

impl crate::writers::ACWritable for EffectsPlayerTeleport {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "EffectsPlayerTeleport").entered();

        write_u16(writer, self.object_teleport_sequence)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

