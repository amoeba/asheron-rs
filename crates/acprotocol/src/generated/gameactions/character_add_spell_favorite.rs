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

// Add a spell to a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddSpellFavorite")]
pub struct CharacterAddSpellFavorite {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
    #[serde(rename = "Index")]
    pub index: u32,
    #[serde(rename = "SpellBar")]
    pub spell_bar: u32,
}

impl crate::readers::ACDataType for CharacterAddSpellFavorite {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterAddSpellFavorite").entered();

        #[cfg(feature = "tracing")]
        let _field_span_spell_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpellId", position = pos).entered()
        };
        let spell_id = LayeredSpellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spell_id);
        #[cfg(feature = "tracing")]
        let _field_span_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Index", position = pos).entered()
        };
        let index = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_index);
        #[cfg(feature = "tracing")]
        let _field_span_spell_bar = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpellBar", position = pos).entered()
        };
        let spell_bar = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spell_bar);

        Ok(Self {
            spell_id,
            index,
            spell_bar,
        })
    }
}

impl crate::writers::ACWritable for CharacterAddSpellFavorite {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterAddSpellFavorite").entered();

        self.spell_id.write(writer)?;
        write_u32(writer, self.index)?;
        write_u32(writer, self.spell_bar)?;
        Ok(())
    }
}

