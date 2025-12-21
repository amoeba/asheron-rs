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

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_GetInscriptionResponse")]
pub struct ItemGetInscriptionResponse {
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
    #[serde(rename = "ScribeAccount")]
    pub scribe_account: String,
}

impl crate::readers::ACDataType for ItemGetInscriptionResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "ItemGetInscriptionResponse").entered();

        #[cfg(feature = "tracing")]
        let _field_span_inscription = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Inscription", position = pos).entered()
        };
        let inscription = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_inscription);
        #[cfg(feature = "tracing")]
        let _field_span_scribe_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ScribeName", position = pos).entered()
        };
        let scribe_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_scribe_name);
        #[cfg(feature = "tracing")]
        let _field_span_scribe_account = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ScribeAccount", position = pos).entered()
        };
        let scribe_account = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_scribe_account);

        Ok(Self {
            inscription,
            scribe_name,
            scribe_account,
        })
    }
}

