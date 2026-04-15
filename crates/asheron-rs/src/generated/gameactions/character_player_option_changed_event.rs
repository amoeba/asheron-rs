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

// Set a single character option.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_PlayerOptionChangedEvent")]
pub struct CharacterPlayerOptionChangedEvent {
    #[serde(rename = "Option")]
    pub option: CharacterOptions1,
    #[serde(rename = "Value")]
    pub value: bool,
}

impl crate::readers::ACDataType for CharacterPlayerOptionChangedEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterPlayerOptionChangedEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_option = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Option", position = pos).entered()
        };
        let option = Ok::<_, Box<dyn std::error::Error>>(CharacterOptions1::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_option);
        #[cfg(feature = "tracing")]
        let _field_span_value = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Value", position = pos).entered()
        };
        let value = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_value);

        Ok(Self {
            option,
            value,
        })
    }
}

impl crate::writers::ACWritable for CharacterPlayerOptionChangedEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CharacterPlayerOptionChangedEvent").entered();

        write_u32(writer, self.option.bits())?;
        write_bool(writer, self.value)?;
        Ok(())
    }
}

