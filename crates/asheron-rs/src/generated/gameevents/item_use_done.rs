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

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UseDone")]
pub struct ItemUseDone {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

impl crate::readers::ACDataType for ItemUseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemUseDone").entered();

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

impl crate::writers::ACWritable for ItemUseDone {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemUseDone").entered();

        write_u32(writer, self.failure_type.clone() as u32)?;
        Ok(())
    }
}

