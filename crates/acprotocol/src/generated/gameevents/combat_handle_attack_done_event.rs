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

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleAttackDoneEvent")]
pub struct CombatHandleAttackDoneEvent {
    #[serde(rename = "Number")]
    pub number: u32,
}

impl crate::readers::ACDataType for CombatHandleAttackDoneEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatHandleAttackDoneEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_number = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Number", position = pos).entered()
        };
        let number = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_number);

        Ok(Self {
            number,
        })
    }
}

