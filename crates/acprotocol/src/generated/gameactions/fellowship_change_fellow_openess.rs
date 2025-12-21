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

// Fellowship Change openness
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_ChangeFellowOpeness")]
pub struct FellowshipChangeFellowOpeness {
    #[serde(rename = "Open")]
    pub open: bool,
}

impl crate::readers::ACDataType for FellowshipChangeFellowOpeness {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipChangeFellowOpeness").entered();

        #[cfg(feature = "tracing")]
        let _field_span_open = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Open", position = pos).entered()
        };
        let open = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_open);

        Ok(Self {
            open,
        })
    }
}

