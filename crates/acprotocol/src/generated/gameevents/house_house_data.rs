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

// House panel information for owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseData")]
pub struct HouseHouseData {
    #[serde(rename = "Data")]
    pub data: HouseData,
}

impl crate::readers::ACDataType for HouseHouseData {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseHouseData").entered();

        #[cfg(feature = "tracing")]
        let _field_span_data = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Data", position = pos).entered()
        };
        let data = HouseData::read(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_data);

        Ok(Self {
            data,
        })
    }
}

impl crate::writers::ACWritable for HouseHouseData {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseHouseData").entered();

        self.data.write(writer)?;
        Ok(())
    }
}

