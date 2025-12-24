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

// Allegiance update cancelled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateAborted")]
pub struct AllegianceAllegianceUpdateAborted {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdateAborted {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceAllegianceUpdateAborted").entered();

        #[cfg(feature = "tracing")]
        let _field_span_failure_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "FailureType", position = pos).entered()
        };
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_failure_type);

        Ok(Self {
            failure_type,
        })
    }
}

impl crate::writers::ACWritable for AllegianceAllegianceUpdateAborted {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceAllegianceUpdateAborted").entered();

        write_u32(writer, self.failure_type.clone() as u32)?;
        Ok(())
    }
}

