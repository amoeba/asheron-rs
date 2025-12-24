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

// Performs a non autonomous jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump_NonAutonomous")]
pub struct MovementJumpNonAutonomous {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

impl crate::readers::ACDataType for MovementJumpNonAutonomous {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementJumpNonAutonomous").entered();

        #[cfg(feature = "tracing")]
        let _field_span_extent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Extent", position = pos).entered()
        };
        let extent = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_extent);

        Ok(Self {
            extent,
        })
    }
}

impl crate::writers::ACWritable for MovementJumpNonAutonomous {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementJumpNonAutonomous").entered();

        write_f32(writer, self.extent)?;
        Ok(())
    }
}

