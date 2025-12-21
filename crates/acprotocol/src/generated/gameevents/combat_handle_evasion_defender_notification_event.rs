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

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionDefenderNotificationEvent")]
pub struct CombatHandleEvasionDefenderNotificationEvent {
    #[serde(rename = "AttackerName")]
    pub attacker_name: String,
}

impl crate::readers::ACDataType for CombatHandleEvasionDefenderNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatHandleEvasionDefenderNotificationEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_attacker_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "AttackerName", position = pos).entered()
        };
        let attacker_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_attacker_name);

        Ok(Self {
            attacker_name,
        })
    }
}

