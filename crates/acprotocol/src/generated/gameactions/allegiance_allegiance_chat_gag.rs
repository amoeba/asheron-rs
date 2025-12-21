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

// Gags a person in allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatGag")]
pub struct AllegianceAllegianceChatGag {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "On")]
    pub on: bool,
}

impl crate::readers::ACDataType for AllegianceAllegianceChatGag {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceAllegianceChatGag").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterName", position = pos).entered()
        };
        let character_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_name);
        #[cfg(feature = "tracing")]
        let _field_span_on = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "On", position = pos).entered()
        };
        let on = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_on);

        Ok(Self {
            character_name,
            on,
        })
    }
}

