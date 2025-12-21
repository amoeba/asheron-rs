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

// Sets the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceName")]
pub struct AllegianceSetAllegianceName {
    #[serde(rename = "Name")]
    pub name: String,
}

impl crate::readers::ACDataType for AllegianceSetAllegianceName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceSetAllegianceName").entered();

        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);

        Ok(Self {
            name,
        })
    }
}

