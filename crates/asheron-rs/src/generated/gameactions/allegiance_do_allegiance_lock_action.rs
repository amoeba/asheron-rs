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

// Perform the allegiance lock action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_DoAllegianceLockAction")]
pub struct AllegianceDoAllegianceLockAction {
    #[serde(rename = "Action")]
    pub action: AllegianceLockAction,
}

impl crate::readers::ACDataType for AllegianceDoAllegianceLockAction {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceDoAllegianceLockAction").entered();

        #[cfg(feature = "tracing")]
        let _field_span_action = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Action", position = pos).entered()
        };
        let action = AllegianceLockAction::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_action);

        Ok(Self {
            action,
        })
    }
}

impl crate::writers::ACWritable for AllegianceDoAllegianceLockAction {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceDoAllegianceLockAction").entered();

        write_u32(writer, self.action.clone() as u32)?;
        Ok(())
    }
}

