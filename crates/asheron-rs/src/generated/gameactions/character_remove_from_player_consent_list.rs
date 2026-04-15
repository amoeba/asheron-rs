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

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveFromPlayerConsentList")]
pub struct CharacterRemoveFromPlayerConsentList {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl crate::readers::ACDataType for CharacterRemoveFromPlayerConsentList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterRemoveFromPlayerConsentList").entered();

        #[cfg(feature = "tracing")]
        let _field_span_target_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TargetName", position = pos).entered()
        };
        let target_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_target_name);

        Ok(Self {
            target_name,
        })
    }
}

impl crate::writers::ACWritable for CharacterRemoveFromPlayerConsentList {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterRemoveFromPlayerConsentList").entered();

        write_string(writer, &self.target_name)?;
        Ok(())
    }
}

