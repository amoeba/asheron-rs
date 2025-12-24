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

// Add an item to the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddShortCut")]
pub struct CharacterAddShortCut {
    #[serde(rename = "Shortcut")]
    pub shortcut: ShortCutData,
}

impl crate::readers::ACDataType for CharacterAddShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterAddShortCut").entered();

        #[cfg(feature = "tracing")]
        let _field_span_shortcut = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Shortcut", position = pos).entered()
        };
        let shortcut = ShortCutData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_shortcut);

        Ok(Self {
            shortcut,
        })
    }
}

impl crate::writers::ACWritable for CharacterAddShortCut {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterAddShortCut").entered();

        self.shortcut.write(writer)?;
        Ok(())
    }
}

