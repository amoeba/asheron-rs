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

// Sends an autonomous position
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomousPosition")]
pub struct MovementAutonomousPosition {
    #[serde(rename = "Position")]
    pub position: AutonomousPositionPack,
}

impl crate::readers::ACDataType for MovementAutonomousPosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementAutonomousPosition").entered();

        #[cfg(feature = "tracing")]
        let _field_span_position = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Position", position = pos).entered()
        };
        let position = AutonomousPositionPack::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_position);

        Ok(Self {
            position,
        })
    }
}

impl crate::writers::ACWritable for MovementAutonomousPosition {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementAutonomousPosition").entered();

        self.position.write(writer)?;
        Ok(())
    }
}

