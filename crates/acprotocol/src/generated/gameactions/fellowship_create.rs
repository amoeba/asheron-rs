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

// Create a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Create")]
pub struct FellowshipCreate {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShareXP")]
    pub share_xp: bool,
}

impl crate::readers::ACDataType for FellowshipCreate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipCreate").entered();

        #[cfg(feature = "tracing")]
        let _field_span_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Name", position = pos).entered()
        };
        let name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_name);
        #[cfg(feature = "tracing")]
        let _field_span_share_xp = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "ShareXP", position = pos).entered()
        };
        let share_xp = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_share_xp);

        Ok(Self {
            name,
            share_xp,
        })
    }
}

