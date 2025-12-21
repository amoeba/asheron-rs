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

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_BreakAllegianceBoot")]
pub struct AllegianceBreakAllegianceBoot {
    #[serde(rename = "BooteeName")]
    pub bootee_name: String,
    #[serde(rename = "AccountBoot")]
    pub account_boot: bool,
}

impl crate::readers::ACDataType for AllegianceBreakAllegianceBoot {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceBreakAllegianceBoot").entered();

        #[cfg(feature = "tracing")]
        let _field_span_bootee_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "BooteeName", position = pos).entered()
        };
        let bootee_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_bootee_name);
        #[cfg(feature = "tracing")]
        let _field_span_account_boot = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AccountBoot", position = pos).entered()
        };
        let account_boot = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_account_boot);

        Ok(Self {
            bootee_name,
            account_boot,
        })
    }
}

