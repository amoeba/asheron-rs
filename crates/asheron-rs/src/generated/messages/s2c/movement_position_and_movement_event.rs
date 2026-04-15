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

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_PositionAndMovementEvent")]
pub struct MovementPositionAndMovementEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Position")]
    pub position: PositionPack,
    #[serde(rename = "MovementData")]
    pub movement_data: MovementData,
}

impl crate::readers::ACDataType for MovementPositionAndMovementEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementPositionAndMovementEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_position = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Position", position = pos).entered()
        };
        let position = PositionPack::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_position);
        #[cfg(feature = "tracing")]
        let _field_span_movement_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "MovementData", position = pos).entered()
        };
        let movement_data = MovementData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_movement_data);

        Ok(Self {
            object_id,
            position,
            movement_data,
        })
    }
}

impl crate::writers::ACWritable for MovementPositionAndMovementEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MovementPositionAndMovementEvent").entered();

        self.object_id.write(writer)?;
        self.position.write(writer)?;
        self.movement_data.write(writer)?;
        Ok(())
    }
}

