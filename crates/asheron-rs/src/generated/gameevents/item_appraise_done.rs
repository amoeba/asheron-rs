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

// Close Assess Panel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_AppraiseDone")]
pub struct ItemAppraiseDone {
    #[serde(rename = "Unknown")]
    pub unknown: u32,
}

impl crate::readers::ACDataType for ItemAppraiseDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemAppraiseDone").entered();

        #[cfg(feature = "tracing")]
        let _field_span_unknown = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Unknown", position = pos).entered()
        };
        let unknown = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_unknown);

        Ok(Self {
            unknown,
        })
    }
}

impl crate::writers::ACWritable for ItemAppraiseDone {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "ItemAppraiseDone").entered();

        write_u32(writer, self.unknown)?;
        Ok(())
    }
}

