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

// Set multiple character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterOptionsEvent")]
pub struct CharacterCharacterOptionsEvent {
    #[serde(rename = "Options")]
    pub options: PlayerModule,
}

impl crate::readers::ACDataType for CharacterCharacterOptionsEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterCharacterOptionsEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_options = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Options", position = pos).entered()
        };
        let options = PlayerModule::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_options);

        Ok(Self {
            options,
        })
    }
}

impl crate::writers::ACWritable for CharacterCharacterOptionsEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterCharacterOptionsEvent").entered();

        self.options.write(writer)?;
        Ok(())
    }
}

