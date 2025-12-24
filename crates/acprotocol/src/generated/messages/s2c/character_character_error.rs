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

// Failure to log in
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterError")]
pub struct CharacterCharacterError {
    #[serde(rename = "Reason")]
    pub reason: CharacterErrorType,
}

impl crate::readers::ACDataType for CharacterCharacterError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterCharacterError").entered();

        #[cfg(feature = "tracing")]
        let _field_span_reason = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Reason", position = pos).entered()
        };
        let reason = CharacterErrorType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_reason);

        Ok(Self {
            reason,
        })
    }
}

impl crate::writers::ACWritable for CharacterCharacterError {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterCharacterError").entered();

        write_u32(writer, self.reason.clone() as u32)?;
        Ok(())
    }
}

