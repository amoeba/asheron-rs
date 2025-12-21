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

// Set or update an Object DId property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateDataId")]
pub struct QualitiesPrivateUpdateDataId {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyDataId,
    #[serde(rename = "Value")]
    pub value: u32,
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateDataId {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "QualitiesPrivateUpdateDataId").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_key = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Key", position = pos).entered()
        };
        let key = PropertyDataId::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_key);
        #[cfg(feature = "tracing")]
        let _field_span_value = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Value", position = pos).entered()
        };
        let value = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_value);

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

