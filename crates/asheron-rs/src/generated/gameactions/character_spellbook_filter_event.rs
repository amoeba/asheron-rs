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

// Changes the spell book filter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SpellbookFilterEvent")]
pub struct CharacterSpellbookFilterEvent {
    #[serde(rename = "Options")]
    pub options: SpellBookFilterOptions,
}

impl crate::readers::ACDataType for CharacterSpellbookFilterEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterSpellbookFilterEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_options = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Options", position = pos).entered()
        };
        let options = Ok::<_, Box<dyn std::error::Error>>(SpellBookFilterOptions::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_options);

        Ok(Self {
            options,
        })
    }
}

impl crate::writers::ACWritable for CharacterSpellbookFilterEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterSpellbookFilterEvent").entered();

        write_u32(writer, self.options.bits())?;
        Ok(())
    }
}

