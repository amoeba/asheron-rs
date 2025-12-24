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

// Lists available house /house available
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ListAvailableHouses")]
pub struct HouseListAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
}

impl crate::readers::ACDataType for HouseListAvailableHouses {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseListAvailableHouses").entered();

        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);

        Ok(Self {
            type_,
        })
    }
}

impl crate::writers::ACWritable for HouseListAvailableHouses {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseListAvailableHouses").entered();

        write_u32(writer, self.type_.clone() as u32)?;
        Ok(())
    }
}

