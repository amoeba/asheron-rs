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

// Sets an allegiance officer title for a given level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceOfficerTitle")]
pub struct AllegianceSetAllegianceOfficerTitle {
    #[serde(rename = "Level")]
    pub level: AllegianceOfficerLevel,
    #[serde(rename = "Title")]
    pub title: String,
}

impl crate::readers::ACDataType for AllegianceSetAllegianceOfficerTitle {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceSetAllegianceOfficerTitle").entered();

        #[cfg(feature = "tracing")]
        let _field_span_level = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Level", position = pos).entered()
        };
        let level = AllegianceOfficerLevel::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_level);
        #[cfg(feature = "tracing")]
        let _field_span_title = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Title", position = pos).entered()
        };
        let title = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_title);

        Ok(Self {
            level,
            title,
        })
    }
}

