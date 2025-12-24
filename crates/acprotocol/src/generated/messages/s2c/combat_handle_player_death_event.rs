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

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Combat_HandlePlayerDeathEvent")]
pub struct CombatHandlePlayerDeathEvent {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "KilledId")]
    pub killed_id: ObjectId,
    #[serde(rename = "KillerId")]
    pub killer_id: ObjectId,
}

impl crate::readers::ACDataType for CombatHandlePlayerDeathEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "CombatHandlePlayerDeathEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_message = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Message", position = pos).entered()
        };
        let message = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_message);
        #[cfg(feature = "tracing")]
        let _field_span_killed_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "KilledId", position = pos).entered()
        };
        let killed_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_killed_id);
        #[cfg(feature = "tracing")]
        let _field_span_killer_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "KillerId", position = pos).entered()
        };
        let killer_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_killer_id);

        Ok(Self {
            message,
            killed_id,
            killer_id,
        })
    }
}

impl crate::writers::ACWritable for CombatHandlePlayerDeathEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "CombatHandlePlayerDeathEvent").entered();

        write_string(writer, &self.message)?;
        self.killed_id.write(writer)?;
        self.killer_id.write(writer)?;
        Ok(())
    }
}

