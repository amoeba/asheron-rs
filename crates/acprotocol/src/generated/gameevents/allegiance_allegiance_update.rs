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

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdate")]
pub struct AllegianceAllegianceUpdate {
    #[serde(rename = "Rank")]
    pub rank: u32,
    #[serde(rename = "Profile")]
    pub profile: AllegianceProfile,
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "AllegianceAllegianceUpdate").entered();

        #[cfg(feature = "tracing")]
        let _field_span_rank = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Rank", position = pos).entered()
        };
        let rank = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rank);
        #[cfg(feature = "tracing")]
        let _field_span_profile = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Profile", position = pos).entered()
        };
        let profile = AllegianceProfile::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_profile);

        Ok(Self {
            rank,
            profile,
        })
    }
}

impl crate::writers::ACWritable for AllegianceAllegianceUpdate {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "AllegianceAllegianceUpdate").entered();

        write_u32(writer, self.rank)?;
        self.profile.write(writer)?;
        Ok(())
    }
}

