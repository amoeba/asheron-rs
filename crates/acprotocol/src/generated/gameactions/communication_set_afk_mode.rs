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

// Set AFK mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetAFKMode")]
pub struct CommunicationSetAFKMode {
    #[serde(rename = "AFK")]
    pub afk: bool,
}

impl crate::readers::ACDataType for CommunicationSetAFKMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationSetAFKMode").entered();

        #[cfg(feature = "tracing")]
        let _field_span_afk = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AFK", position = pos).entered()
        };
        let afk = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_afk);

        Ok(Self {
            afk,
        })
    }
}

