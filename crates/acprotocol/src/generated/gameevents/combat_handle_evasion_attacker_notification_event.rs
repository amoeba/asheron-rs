use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandleEvasionAttackerNotificationEvent")]
pub struct CombatHandleEvasionAttackerNotificationEvent {
    #[serde(rename = "DefenderName")]
    pub defender_name: String,
}

impl crate::readers::ACDataType for CombatHandleEvasionAttackerNotificationEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatHandleEvasionAttackerNotificationEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_defender_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "DefenderName", position = pos).entered()
        };
        let defender_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_defender_name);

        Ok(Self {
            defender_name,
        })
    }
}

impl crate::writers::ACWritable for CombatHandleEvasionAttackerNotificationEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CombatHandleEvasionAttackerNotificationEvent").entered();

        write_string(writer, &self.defender_name)?;
        Ok(())
    }
}

