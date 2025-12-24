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

// Removes an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_RemoveAllegianceOfficer")]
pub struct AllegianceRemoveAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
}

impl crate::readers::ACDataType for AllegianceRemoveAllegianceOfficer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceRemoveAllegianceOfficer").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterName", position = pos).entered()
        };
        let character_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_name);

        Ok(Self {
            character_name,
        })
    }
}

impl crate::writers::ACWritable for AllegianceRemoveAllegianceOfficer {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceRemoveAllegianceOfficer").entered();

        write_string(writer, &self.character_name)?;
        Ok(())
    }
}

