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

// Display a confirmation panel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationRequest")]
pub struct CharacterConfirmationRequest {
    #[serde(rename = "ConfirmationType")]
    pub confirmation_type: ConfirmationType,
    #[serde(rename = "ContextId")]
    pub context_id: u32,
    #[serde(rename = "Text")]
    pub text: String,
}

impl crate::readers::ACDataType for CharacterConfirmationRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterConfirmationRequest").entered();

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
        #[cfg(feature = "tracing")]
        let _field_span_text = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Text", position = pos).entered()
        };
        let text = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_text);

        Ok(Self {
            confirmation_type,
            context_id,
            text,
        })
    }
}

