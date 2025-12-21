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

// DDD request for data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_RequestDataMessage")]
pub struct DDDRequestDataMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
}

impl crate::readers::ACDataType for DDDRequestDataMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDRequestDataMessage").entered();

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

        Ok(Self {
            resource_type,
            resource_id,
        })
    }
}

