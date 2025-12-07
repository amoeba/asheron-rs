use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// Update Rent Payment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentPayment")]
pub struct HouseUpdateRentPayment {
    #[serde(rename = "Rent")]
    pub rent: PackableList<HousePayment>,
}

impl HouseUpdateRentPayment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let rent = read_packable_list::<HousePayment>(reader)?;

        Ok(Self {
            rent,
        })
    }
}

impl crate::readers::ACDataType for HouseUpdateRentPayment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseUpdateRentPayment::read(reader)
    }
}

