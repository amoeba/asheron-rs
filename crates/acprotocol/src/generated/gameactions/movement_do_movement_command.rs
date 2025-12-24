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

// Performs a movement based on input
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_DoMovementCommand")]
pub struct MovementDoMovementCommand {
    #[serde(rename = "Motion")]
    pub motion: u32,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "HoldKey")]
    pub hold_key: HoldKey,
}

impl crate::readers::ACDataType for MovementDoMovementCommand {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementDoMovementCommand").entered();

        #[cfg(feature = "tracing")]
        let _field_span_motion = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Motion", position = pos).entered()
        };
        let motion = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_motion);
        #[cfg(feature = "tracing")]
        let _field_span_speed = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Speed", position = pos).entered()
        };
        let speed = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_speed);
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
            speed,
            hold_key,
        })
    }
}

impl crate::writers::ACWritable for MovementDoMovementCommand {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementDoMovementCommand").entered();

        write_u32(writer, self.motion)?;
        write_f32(writer, self.speed)?;
        write_u32(writer, self.hold_key.clone() as u32)?;
        Ok(())
    }
}

