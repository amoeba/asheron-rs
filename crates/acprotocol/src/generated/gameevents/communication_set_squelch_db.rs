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

// Squelch and Filter List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_SetSquelchDB")]
pub struct CommunicationSetSquelchDB {
    #[serde(rename = "SquelchDB")]
    pub squelch_db: SquelchDB,
}

impl crate::readers::ACDataType for CommunicationSetSquelchDB {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CommunicationSetSquelchDB").entered();

        #[cfg(feature = "tracing")]
        let _field_span_squelch_db = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "SquelchDB", position = pos).entered()
        };
        let squelch_db = SquelchDB::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_squelch_db);

        Ok(Self {
            squelch_db,
        })
    }
}

