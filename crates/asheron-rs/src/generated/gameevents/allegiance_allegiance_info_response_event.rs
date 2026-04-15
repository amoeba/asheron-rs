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

// Returns data for a player's allegiance information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoResponseEvent")]
pub struct AllegianceAllegianceInfoResponseEvent {
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoResponseEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceAllegianceInfoResponseEvent").entered();

        #[cfg(feature = "tracing")]
        let _field_span_target_id = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "TargetId", position = pos).entered()
        };
        let target_id = ObjectId::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_target_id);
        #[cfg(feature = "tracing")]
        let _field_span_profile = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Profile", position = pos).entered()
        };
        let profile = AllegianceProfile::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_profile);

        Ok(Self {
            target_id,
            profile,
        })
    }
}

impl crate::writers::ACWritable for AllegianceAllegianceInfoResponseEvent {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceAllegianceInfoResponseEvent").entered();

        self.target_id.write(writer)?;
        self.profile.write(writer)?;
        Ok(())
    }
}

