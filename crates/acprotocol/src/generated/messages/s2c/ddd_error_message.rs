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

// DDD error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_ErrorMessage")]
pub struct DDDErrorMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "RError")]
    pub r_error: u32,
}

impl crate::readers::ACDataType for DDDErrorMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDErrorMessage").entered();

        #[cfg(feature = "tracing")]
        let _field_span_resource_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ResourceType", position = pos).entered()
        };
        let resource_type = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_resource_type);
        #[cfg(feature = "tracing")]
        let _field_span_resource_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ResourceId", position = pos).entered()
        };
        let resource_id = DataId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_resource_id);
        #[cfg(feature = "tracing")]
        let _field_span_r_error = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RError", position = pos).entered()
        };
        let r_error = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_r_error);

        Ok(Self {
            resource_type,
            resource_id,
            r_error,
        })
    }
}

impl crate::writers::ACWritable for DDDErrorMessage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "DDDErrorMessage").entered();

        write_u32(writer, self.resource_type)?;
        self.resource_id.write(writer)?;
        write_u32(writer, self.r_error)?;
        Ok(())
    }
}

