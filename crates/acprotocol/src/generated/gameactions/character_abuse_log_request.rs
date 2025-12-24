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

// Sends an abuse report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AbuseLogRequest")]
pub struct CharacterAbuseLogRequest {
    #[serde(rename = "Character")]
    pub character: String,
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Complaint")]
    pub complaint: String,
}

impl crate::readers::ACDataType for CharacterAbuseLogRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterAbuseLogRequest").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Character", position = pos).entered()
        };
        let character = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character);
        #[cfg(feature = "tracing")]
        let _field_span_status = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Status", position = pos).entered()
        };
        let status = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_status);
        #[cfg(feature = "tracing")]
        let _field_span_complaint = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Complaint", position = pos).entered()
        };
        let complaint = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_complaint);

        Ok(Self {
            character,
            status,
            complaint,
        })
    }
}

impl crate::writers::ACWritable for CharacterAbuseLogRequest {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterAbuseLogRequest").entered();

        write_string(writer, &self.character)?;
        write_u32(writer, self.status)?;
        write_string(writer, &self.complaint)?;
        Ok(())
    }
}

