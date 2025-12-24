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

// Quit the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Quit")]
pub struct FellowshipQuit {
    #[serde(rename = "Disband")]
    pub disband: bool,
}

impl crate::readers::ACDataType for FellowshipQuit {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "FellowshipQuit").entered();

        #[cfg(feature = "tracing")]
        let _field_span_disband = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Disband", position = pos).entered()
        };
        let disband = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_disband);

        Ok(Self {
            disband,
        })
    }
}

impl crate::writers::ACWritable for FellowshipQuit {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "FellowshipQuit").entered();

        write_bool(writer, self.disband)?;
        Ok(())
    }
}

