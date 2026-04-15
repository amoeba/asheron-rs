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

// Sets an autonomy level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomyLevel")]
pub struct MovementAutonomyLevel {
    #[serde(rename = "AutonomyLevel")]
    pub autonomy_level: u32,
}

impl crate::readers::ACDataType for MovementAutonomyLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementAutonomyLevel").entered();

        #[cfg(feature = "tracing")]
        let _field_span_autonomy_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AutonomyLevel", position = pos).entered()
        };
        let autonomy_level = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_autonomy_level);
        #[cfg(feature = "tracing")]
        let _field_span_alignment_marker_align_dword = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "__alignment_marker_align_dword", position = pos).entered()
        };
        align_dword(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_alignment_marker_align_dword);

        Ok(Self {
            autonomy_level,
        })
    }
}

impl crate::writers::ACWritable for MovementAutonomyLevel {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementAutonomyLevel").entered();

        write_u32(writer, self.autonomy_level)?;
        align_dword_write(writer)?;
        Ok(())
    }
}

