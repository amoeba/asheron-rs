use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_VectorUpdate")]
pub struct MovementVectorUpdate {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Velocity")]
    pub velocity: Vector3,
    #[serde(rename = "Omega")]
    pub omega: Vector3,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectVectorSequence")]
    pub object_vector_sequence: u16,
}

impl crate::readers::ACDataType for MovementVectorUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MovementVectorUpdate").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_velocity = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Velocity", position = pos).entered()
        };
        let velocity = Vector3::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_velocity);
        #[cfg(feature = "tracing")]
        let _field_span_omega = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Omega", position = pos).entered()
        };
        let omega = Vector3::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_omega);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_vector_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectVectorSequence", position = pos).entered()
        };
        let object_vector_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_vector_sequence);

        Ok(Self {
            object_id,
            velocity,
            omega,
            object_instance_sequence,
            object_vector_sequence,
        })
    }
}

