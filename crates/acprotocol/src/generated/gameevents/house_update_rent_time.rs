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

// Update Rent Time
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentTime")]
pub struct HouseUpdateRentTime {
    #[serde(rename = "RentTime")]
    pub rent_time: u32,
}

impl crate::readers::ACDataType for HouseUpdateRentTime {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseUpdateRentTime").entered();

        #[cfg(feature = "tracing")]
        let _field_span_rent_time = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "RentTime", position = pos).entered()
        };
        let rent_time = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rent_time);

        Ok(Self {
            rent_time,
        })
    }
}

