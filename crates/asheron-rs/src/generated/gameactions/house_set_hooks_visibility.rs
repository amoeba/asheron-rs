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

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetHooksVisibility")]
pub struct HouseSetHooksVisibility {
    #[serde(rename = "Visible")]
    pub visible: bool,
}

impl crate::readers::ACDataType for HouseSetHooksVisibility {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseSetHooksVisibility").entered();

        #[cfg(feature = "tracing")]
        let _field_span_visible = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Visible", position = pos).entered()
        };
        let visible = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_visible);

        Ok(Self {
            visible,
        })
    }
}

impl crate::writers::ACWritable for HouseSetHooksVisibility {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseSetHooksVisibility").entered();

        write_bool(writer, self.visible)?;
        Ok(())
    }
}

