use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Adds a guest to your house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AddPermanentGuest")]
pub struct HouseAddPermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

impl crate::readers::ACDataType for HouseAddPermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

