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

// TODO
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationResponseMessage")]
pub struct DDDInterrogationResponseMessage {
    #[serde(rename = "Language")]
    pub language: u32,
    #[serde(rename = "Files")]
    pub files: PackableList<i64>,
}

impl crate::readers::ACDataType for DDDInterrogationResponseMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "DDDInterrogationResponseMessage").entered();

        #[cfg(feature = "tracing")]
        let _field_span_language = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Language", position = pos).entered()
        };
        let language = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_language);
        #[cfg(feature = "tracing")]
        let _field_span_files = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Files", position = pos).entered()
        };
        let files = read_packable_list::<i64>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_files);

        Ok(Self {
            language,
            files,
        })
    }
}

impl crate::writers::ACWritable for DDDInterrogationResponseMessage {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "DDDInterrogationResponseMessage").entered();

        write_u32(writer, self.language)?;
        write_packable_list::<i64>(writer, &self.files)?;
        Ok(())
    }
}

