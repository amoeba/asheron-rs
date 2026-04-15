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

// Create or join a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FullUpdate")]
pub struct FellowshipFullUpdate {
    #[serde(rename = "Fellowship")]
    pub fellowship: Fellowship,
}

impl crate::readers::ACDataType for FellowshipFullUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipFullUpdate").entered();

        #[cfg(feature = "tracing")]
        let _field_span_fellowship = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Fellowship", position = pos).entered()
        };
        let fellowship = Fellowship::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_fellowship);

        Ok(Self {
            fellowship,
        })
    }
}

impl crate::writers::ACWritable for FellowshipFullUpdate {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FellowshipFullUpdate").entered();

        self.fellowship.write(writer)?;
        Ok(())
    }
}

