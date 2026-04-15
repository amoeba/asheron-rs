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

// Confirmation done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationDone")]
pub struct CharacterConfirmationDone {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
}

impl crate::readers::ACDataType for CharacterConfirmationDone {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterConfirmationDone").entered();

        #[cfg(feature = "tracing")]
        let _field_span_confirmation_type = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ConfirmationType", position = pos).entered()
        };
        let confirmation_type = ConfirmationType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_confirmation_type);
        #[cfg(feature = "tracing")]
        let _field_span_context_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ContextId", position = pos).entered()
        };
        let context_id = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_context_id);

        Ok(Self {
            confirmation_type,
            context_id,
        })
    }
}

impl crate::writers::ACWritable for CharacterConfirmationDone {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterConfirmationDone").entered();

        write_u32(writer, self.confirmation_type.clone() as u32)?;
        write_u32(writer, self.context_id)?;
        Ok(())
    }
}

