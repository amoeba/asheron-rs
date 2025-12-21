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

// Changes the combat mode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_ChangeCombatMode")]
pub struct CombatChangeCombatMode {
    #[serde(rename = "Mode")]
    pub mode: CombatMode,
}

impl crate::readers::ACDataType for CombatChangeCombatMode {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatChangeCombatMode").entered();

        #[cfg(feature = "tracing")]
        let _field_span_mode = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Mode", position = pos).entered()
        };
        let mode = Ok::<_, Box<dyn std::error::Error>>(CombatMode::from_bits_retain(read_u32(reader)?))?;
        #[cfg(feature = "tracing")]
        drop(_field_span_mode);

        Ok(Self {
            mode,
        })
    }
}

