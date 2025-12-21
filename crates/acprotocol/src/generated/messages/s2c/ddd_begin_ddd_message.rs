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

// A list of dat files that need to be patched
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_BeginDDDMessage")]
pub struct DDDBeginDDDMessage {
    #[serde(rename = "DataExpected")]
    pub data_expected: u32,
    #[serde(rename = "Revisions")]
    pub revisions: PackableList<DDDRevision>,
}

impl crate::readers::ACDataType for DDDBeginDDDMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDBeginDDDMessage").entered();

        #[cfg(feature = "tracing")]
        let _field_span_data_expected = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DataExpected", position = pos).entered()
        };
        let data_expected = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_data_expected);
        #[cfg(feature = "tracing")]
        let _field_span_revisions = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Revisions", position = pos).entered()
        };
        let revisions = read_packable_list::<DDDRevision>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_revisions);

        Ok(Self {
            data_expected,
            revisions,
        })
    }
}

