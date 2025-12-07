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

// Removes a specific player from your house guest list, /house guest remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RemovePermanentGuest")]
pub struct HouseRemovePermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

impl HouseRemovePermanentGuest {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

impl crate::readers::ACDataType for HouseRemovePermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRemovePermanentGuest::read(reader)
    }
}

