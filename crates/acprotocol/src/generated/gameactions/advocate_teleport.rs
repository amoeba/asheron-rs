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

// Advocate Teleport
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Advocate_Teleport")]
pub struct AdvocateTeleport {
    #[serde(rename = "ObjectId")]
    pub object_id: String,
    #[serde(rename = "Destination")]
    pub destination: Position,
}

impl crate::readers::ACDataType for AdvocateTeleport {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AdvocateTeleport").entered();

        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_destination = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Destination", position = pos).entered()
        };
        let destination = Position::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_destination);

        Ok(Self {
            object_id,
            destination,
        })
    }
}

