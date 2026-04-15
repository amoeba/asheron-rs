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

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ModifyAccountSquelch")]
pub struct CommunicationModifyAccountSquelch {
    #[serde(rename = "Add")]
    pub add: bool,
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl crate::readers::ACDataType for CommunicationModifyAccountSquelch {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationModifyAccountSquelch").entered();

        #[cfg(feature = "tracing")]
        let _field_span_add = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Add", position = pos).entered()
        };
        let add = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_add);
        #[cfg(feature = "tracing")]
        let _field_span_character_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterName", position = pos).entered()
        };
        let character_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_name);

        Ok(Self {
            add,
            character_name,
        })
    }
}

impl crate::writers::ACWritable for CommunicationModifyAccountSquelch {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CommunicationModifyAccountSquelch").entered();

        write_bool(writer, self.add)?;
        write_string(writer, &self.character_name)?;
        Ok(())
    }
}

