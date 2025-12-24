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

// Performs a jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump")]
pub struct MovementJump {
    #[serde(rename = "Jump")]
    pub jump: JumpPack,
}

impl crate::readers::ACDataType for MovementJump {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementJump").entered();

        #[cfg(feature = "tracing")]
        let _field_span_jump = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Jump", position = pos).entered()
        };
        let jump = JumpPack::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_jump);

        Ok(Self {
            jump,
        })
    }
}

impl crate::writers::ACWritable for MovementJump {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementJump").entered();

        self.jump.write(writer)?;
        Ok(())
    }
}

