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

// QueryHealthResponse: Update a creature's health bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_QueryHealthResponse")]
pub struct CombatQueryHealthResponse {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Health")]
    pub health: f32,
}

impl crate::readers::ACDataType for CombatQueryHealthResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatQueryHealthResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_health = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Health", position = pos).entered()
        };
        let health = read_f32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_health);

        Ok(Self {
            object_id,
            health,
        })
    }
}

impl crate::writers::ACWritable for CombatQueryHealthResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CombatQueryHealthResponse").entered();

        self.object_id.write(writer)?;
        write_f32(writer, self.health)?;
        Ok(())
    }
}

