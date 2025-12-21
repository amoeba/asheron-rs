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

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetState")]
pub struct ItemSetState {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "NewState")]
    pub new_state: PhysicsState,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectStateSequence")]
    pub object_state_sequence: u16,
}

impl crate::readers::ACDataType for ItemSetState {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemSetState").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_new_state = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NewState", position = pos).entered()
        };
        let new_state = Ok::<_, Box<dyn std::error::Error>>(PhysicsState::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_new_state);
        #[cfg(feature = "tracing")]
        let _field_span_object_instance_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectInstanceSequence", position = pos).entered()
        };
        let object_instance_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_instance_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_state_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectStateSequence", position = pos).entered()
        };
        let object_state_sequence = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_state_sequence);

        Ok(Self {
            object_id,
            new_state,
            object_instance_sequence,
            object_state_sequence,
        })
    }
}

