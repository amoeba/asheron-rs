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

// Adds a guest to your house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AddPermanentGuest")]
pub struct HouseAddPermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

impl crate::readers::ACDataType for HouseAddPermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseAddPermanentGuest").entered();

        #[cfg(feature = "tracing")]
        let _field_span_guest_name = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GuestName", position = pos).entered()
        };
        let guest_name = read_string(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_guest_name);

        Ok(Self {
            guest_name,
        })
    }
}

impl crate::writers::ACWritable for HouseAddPermanentGuest {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseAddPermanentGuest").entered();

        write_string(writer, &self.guest_name)?;
        Ok(())
    }
}

