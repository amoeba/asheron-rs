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

// Set a title for the current character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_AddOrSetCharacterTitle")]
pub struct SocialAddOrSetCharacterTitle {
    #[serde(rename = "NewTitle")]
    pub new_title: u32,
    #[serde(rename = "SetAsDisplayTitle")]
    pub set_as_display_title: bool,
}

impl crate::readers::ACDataType for SocialAddOrSetCharacterTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "SocialAddOrSetCharacterTitle").entered();

        #[cfg(feature = "tracing")]
        let _field_span_new_title = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NewTitle", position = pos).entered()
        };
        let new_title = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_new_title);
        #[cfg(feature = "tracing")]
        let _field_span_set_as_display_title = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SetAsDisplayTitle", position = pos).entered()
        };
        let set_as_display_title = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_set_as_display_title);

        Ok(Self {
            new_title,
            set_as_display_title,
        })
    }
}

impl crate::writers::ACWritable for SocialAddOrSetCharacterTitle {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "SocialAddOrSetCharacterTitle").entered();

        write_u32(writer, self.new_title)?;
        write_bool(writer, self.set_as_display_title)?;
        Ok(())
    }
}

