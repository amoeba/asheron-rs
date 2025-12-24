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

// Remove an item from the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveShortCut")]
pub struct CharacterRemoveShortCut {
    #[serde(rename = "Index")]
    pub index: u32,
}

impl crate::readers::ACDataType for CharacterRemoveShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterRemoveShortCut").entered();

        #[cfg(feature = "tracing")]
        let _field_span_index = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Index", position = pos).entered()
        };
        let index = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_index);

        Ok(Self {
            index,
        })
    }
}

impl crate::writers::ACWritable for CharacterRemoveShortCut {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterRemoveShortCut").entered();

        write_u32(writer, self.index)?;
        Ok(())
    }
}

