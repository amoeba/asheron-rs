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

// Character creation result
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SendCharGenResult")]
pub struct CharacterSendCharGenResult {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Result")]
    pub result: CharGenResult,
}

impl crate::readers::ACDataType for CharacterSendCharGenResult {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CharacterSendCharGenResult").entered();

        #[cfg(feature = "tracing")]
        let _field_span_account = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Account", position = pos).entered()
        };
        let account = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account);
        #[cfg(feature = "tracing")]
        let _field_span_result = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Result", position = pos).entered()
        };
        let result = CharGenResult::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_result);

        Ok(Self {
            account,
            result,
        })
    }
}

