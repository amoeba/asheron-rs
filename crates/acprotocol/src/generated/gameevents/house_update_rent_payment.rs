use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

