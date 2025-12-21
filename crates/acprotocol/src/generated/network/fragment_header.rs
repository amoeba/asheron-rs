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

// Header for packet fragments used in network communication
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FragmentHeader {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "Index")]
    pub index: u16,
}

impl crate::readers::ACDataType for FragmentHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FragmentHeader").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Id", position = pos).entered()
        };
        let id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_id);
        #[cfg(feature = "tracing")]
        let _field_span_count = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Count", position = pos).entered()
        };
        let count = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_count);
        #[cfg(feature = "tracing")]
        let _field_span_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Index", position = pos).entered()
        };
        let index = read_u16(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_index);

        Ok(Self {
            sequence,
            id,
            count,
            index,
        })
    }
}

