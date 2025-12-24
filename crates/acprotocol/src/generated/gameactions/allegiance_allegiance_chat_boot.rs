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

// Boots a player from the allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceChatBoot")]
pub struct AllegianceAllegianceChatBoot {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Reason")]
    pub reason: String,
}

impl crate::readers::ACDataType for AllegianceAllegianceChatBoot {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceAllegianceChatBoot").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterName", position = pos).entered()
        };
        let character_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_name);
        #[cfg(feature = "tracing")]
        let _field_span_reason = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Reason", position = pos).entered()
        };
        let reason = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_reason);

        Ok(Self {
            character_name,
            reason,
        })
    }
}

impl crate::writers::ACWritable for AllegianceAllegianceChatBoot {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceAllegianceChatBoot").entered();

        write_string(writer, &self.character_name)?;
        write_string(writer, &self.reason)?;
        Ok(())
    }
}

