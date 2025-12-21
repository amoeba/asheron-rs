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

// House Guest List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateHAR")]
pub struct HouseUpdateHAR {
    #[serde(rename = "GuestList")]
    pub guest_list: HAR,
}

impl crate::readers::ACDataType for HouseUpdateHAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseUpdateHAR").entered();

        #[cfg(feature = "tracing")]
        let _field_span_guest_list = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "GuestList", position = pos).entered()
        };
        let guest_list = HAR::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_guest_list);

        Ok(Self {
            guest_list,
        })
    }
}

