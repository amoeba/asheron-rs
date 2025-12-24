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

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateFellow")]
pub struct FellowshipUpdateFellow {
    #[serde(rename = "Fellow")]
    pub fellow: Fellow,
    #[serde(rename = "UpdateType")]
    pub update_type: FellowUpdateType,
}

impl crate::readers::ACDataType for FellowshipUpdateFellow {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipUpdateFellow").entered();

        #[cfg(feature = "tracing")]
        let _field_span_fellow = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Fellow", position = pos).entered()
        };
        let fellow = Fellow::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_fellow);
        #[cfg(feature = "tracing")]
        let _field_span_update_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "UpdateType", position = pos).entered()
        };
        let update_type = FellowUpdateType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_update_type);

        Ok(Self {
            fellow,
            update_type,
        })
    }
}

impl crate::writers::ACWritable for FellowshipUpdateFellow {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FellowshipUpdateFellow").entered();

        self.fellow.write(writer)?;
        write_u32(writer, self.update_type.clone() as u32)?;
        Ok(())
    }
}

