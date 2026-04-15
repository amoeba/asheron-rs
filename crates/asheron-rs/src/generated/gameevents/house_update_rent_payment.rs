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

// Update Rent Payment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentPayment")]
pub struct HouseUpdateRentPayment {
    #[serde(rename = "Rent")]
    pub rent: PackableList<HousePayment>,
}

impl crate::readers::ACDataType for HouseUpdateRentPayment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "HouseUpdateRentPayment").entered();

        #[cfg(feature = "tracing")]
        let _field_span_rent = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Rent", position = pos).entered()
        };
        let rent = read_packable_list::<HousePayment>(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_rent);

        Ok(Self {
            rent,
        })
    }
}

impl crate::writers::ACWritable for HouseUpdateRentPayment {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "HouseUpdateRentPayment").entered();

        write_packable_list::<HousePayment>(writer, &self.rent)?;
        Ok(())
    }
}

