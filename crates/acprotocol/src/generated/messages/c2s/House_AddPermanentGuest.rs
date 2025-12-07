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

// Adds a guest to your house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AddPermanentGuest")]
pub struct HouseAddPermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

impl HouseAddPermanentGuest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

impl crate::readers::ACDataType for HouseAddPermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAddPermanentGuest::read(reader)
    }
}

