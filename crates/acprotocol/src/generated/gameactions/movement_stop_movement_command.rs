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

// Stops a movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_StopMovementCommand")]
pub struct MovementStopMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

impl crate::readers::ACDataType for MovementStopMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementStopMovementCommand").entered();

        #[cfg(feature = "tracing")]
        let _field_span_motion = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Motion", position = pos).entered()
        };
        let motion = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_motion);
        #[cfg(feature = "tracing")]
        let _field_span_hold_key = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "HoldKey", position = pos).entered()
        };
        let hold_key = HoldKey::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_hold_key);

        Ok(Self {
            motion,
            hold_key,
        })
    }
}

impl crate::writers::ACWritable for MovementStopMovementCommand {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementStopMovementCommand").entered();

        write_u32(writer, self.motion)?;
        write_u32(writer, self.hold_key.clone() as u32)?;
        Ok(())
    }
}

