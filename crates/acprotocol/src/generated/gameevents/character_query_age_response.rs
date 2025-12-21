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

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_QueryAgeResponse")]
pub struct CharacterQueryAgeResponse {
    #[serde(rename = "TargetName")]
    pub target_name: String,
    #[serde(rename = "Age")]
    pub age: String,
}

impl crate::readers::ACDataType for CharacterQueryAgeResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterQueryAgeResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_target_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TargetName", position = pos).entered()
        };
        let target_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_target_name);
        #[cfg(feature = "tracing")]
        let _field_span_age = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Age", position = pos).entered()
        };
        let age = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_age);

        Ok(Self {
            target_name,
            age,
        })
    }
}

