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

// Confirms a dialog
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationResponse")]
pub struct CharacterConfirmationResponse {
    #[serde(rename = "Type")]
    pub type_: ConfirmationType,
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Accepted")]
    pub accepted: bool,
}

impl crate::readers::ACDataType for CharacterConfirmationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterConfirmationResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = ConfirmationType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_context = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Context", position = pos).entered()
        };
        let context = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_context);
        #[cfg(feature = "tracing")]
        let _field_span_accepted = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Accepted", position = pos).entered()
        };
        let accepted = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_accepted);

        Ok(Self {
            type_,
            context,
            accepted,
        })
    }
}

impl crate::writers::ACWritable for CharacterConfirmationResponse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterConfirmationResponse").entered();

        write_u32(writer, self.type_.clone() as u32)?;
        write_u32(writer, self.context)?;
        write_bool(writer, self.accepted)?;
        Ok(())
    }
}

