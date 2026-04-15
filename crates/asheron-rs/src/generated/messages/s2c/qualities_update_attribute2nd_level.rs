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

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute2ndLevel")]
pub struct QualitiesUpdateAttribute2ndLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: CurVitalId,
    #[serde(rename = "Value")]
    pub value: u32,
}

impl crate::readers::ACDataType for QualitiesUpdateAttribute2ndLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "QualitiesUpdateAttribute2ndLevel").entered();

        #[cfg(feature = "tracing")]
        let _field_span_sequence = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Sequence", position = pos).entered()
        };
        let sequence = read_u8(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_sequence);
        #[cfg(feature = "tracing")]
        let _field_span_object_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ObjectId", position = pos).entered()
        };
        let object_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_object_id);
        #[cfg(feature = "tracing")]
        let _field_span_key = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Key", position = pos).entered()
        };
        let key = CurVitalId::try_from(read_u32(reader)?)?;
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
            object_id,
            key,
            value,
        })
    }
}

impl crate::writers::ACWritable for QualitiesUpdateAttribute2ndLevel {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "QualitiesUpdateAttribute2ndLevel").entered();

        write_u8(writer, self.sequence)?;
        self.object_id.write(writer)?;
        write_u32(writer, self.key.clone() as u32)?;
        write_u32(writer, self.value)?;
        Ok(())
    }
}

