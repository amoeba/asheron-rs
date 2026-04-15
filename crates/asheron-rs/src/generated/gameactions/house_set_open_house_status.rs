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

// Sets your house open status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetOpenHouseStatus")]
pub struct HouseSetOpenHouseStatus {
    #[serde(rename = "OpenHouse")]
    pub open_house: bool,
}

impl crate::readers::ACDataType for HouseSetOpenHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseSetOpenHouseStatus").entered();

        #[cfg(feature = "tracing")]
        let _field_span_open_house = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "OpenHouse", position = pos).entered()
        };
        let open_house = read_bool(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_open_house);

        Ok(Self {
            open_house,
        })
    }
}

impl crate::writers::ACWritable for HouseSetOpenHouseStatus {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseSetOpenHouseStatus").entered();

        write_bool(writer, self.open_house)?;
        Ok(())
    }
}

