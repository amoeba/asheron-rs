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

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AvailableHouses")]
pub struct HouseAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
    #[serde(rename = "Houses")]
    pub houses: PackableList<u32>,
    #[serde(rename = "NumHouses")]
    pub num_houses: i32,
}

impl crate::readers::ACDataType for HouseAvailableHouses {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseAvailableHouses").entered();

        #[cfg(feature = "tracing")]
        let _field_span_type_ = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Type", position = pos).entered()
        };
        let type_ = HouseType::try_from(read_u32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_type_);
        #[cfg(feature = "tracing")]
        let _field_span_houses = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Houses", position = pos).entered()
        };
        let houses = read_packable_list::<u32>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_houses);
        #[cfg(feature = "tracing")]
        let _field_span_num_houses = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "NumHouses", position = pos).entered()
        };
        let num_houses = read_i32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_num_houses);

        Ok(Self {
            type_,
            houses,
            num_houses,
        })
    }
}

impl crate::writers::ACWritable for HouseAvailableHouses {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseAvailableHouses").entered();

        write_u32(writer, self.type_.clone() as u32)?;
        write_packable_list::<u32>(writer, &self.houses)?;
        write_i32(writer, self.num_houses)?;
        Ok(())
    }
}

