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

// Update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_UpdateRequest")]
pub struct FellowshipUpdateRequest {
    #[serde(rename = "On")]
    pub on: bool,
}

impl crate::readers::ACDataType for FellowshipUpdateRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipUpdateRequest").entered();

        #[cfg(feature = "tracing")]
        let _field_span_on = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "On", position = pos).entered()
        };
        let on = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_on);

        Ok(Self {
            on,
        })
    }
}

impl crate::writers::ACWritable for FellowshipUpdateRequest {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FellowshipUpdateRequest").entered();

        write_bool(writer, self.on)?;
        Ok(())
    }
}

