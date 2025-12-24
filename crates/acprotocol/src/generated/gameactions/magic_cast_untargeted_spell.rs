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

// Cast a spell with no target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Magic_CastUntargetedSpell")]
pub struct MagicCastUntargetedSpell {
    #[serde(rename = "SpellId")]
    pub spell_id: LayeredSpellId,
}

impl crate::readers::ACDataType for MagicCastUntargetedSpell {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "MagicCastUntargetedSpell").entered();

        #[cfg(feature = "tracing")]
        let _field_span_spell_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SpellId", position = pos).entered()
        };
        let spell_id = LayeredSpellId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_spell_id);

        Ok(Self {
            spell_id,
        })
    }
}

impl crate::writers::ACWritable for MagicCastUntargetedSpell {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "MagicCastUntargetedSpell").entered();

        self.spell_id.write(writer)?;
        Ok(())
    }
}

