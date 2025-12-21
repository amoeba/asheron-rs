use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Sets an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficer")]
pub struct AllegianceSetAllegianceOfficer {
    #[serde(rename = "CharacterName")]
    pub character_name: String,
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
}

impl crate::readers::ACDataType for AllegianceSetAllegianceOfficer {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceSetAllegianceOfficer").entered();

        #[cfg(feature = "tracing")]
        let _field_span_character_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "CharacterName", position = pos).entered()
        };
        let character_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_character_name);
        #[cfg(feature = "tracing")]
        let _field_span_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Level", position = pos).entered()
        };
        let level = AllegianceOfficerLevel::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_level);

        Ok(Self {
            character_name,
            level,
        })
    }
}

