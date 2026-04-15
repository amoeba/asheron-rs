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

// Titles for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_CharacterTitleTable")]
pub struct SocialCharacterTitleTable {
    #[serde(rename = "DisplayTitle")]
    pub display_title: u32,
    #[serde(rename = "Titles")]
    pub titles: PackableList<u32>,
}

impl crate::readers::ACDataType for SocialCharacterTitleTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialCharacterTitleTable").entered();

        #[cfg(feature = "tracing")]
        let _field_span_display_title = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DisplayTitle", position = pos).entered()
        };
        let display_title = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_display_title);
        #[cfg(feature = "tracing")]
        let _field_span_titles = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Titles", position = pos).entered()
        };
        let titles = read_packable_list::<u32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_titles);

        Ok(Self {
            display_title,
            titles,
        })
    }
}

impl crate::writers::ACWritable for SocialCharacterTitleTable {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SocialCharacterTitleTable").entered();

        write_u32(writer, self.display_title)?;
        write_packable_list::<u32>(writer, &self.titles)?;
        Ok(())
    }
}

