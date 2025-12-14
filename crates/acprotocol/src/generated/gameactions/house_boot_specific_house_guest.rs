use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Boots a specific player from your house /house boot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_BootSpecificHouseGuest")]
pub struct HouseBootSpecificHouseGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

impl crate::readers::ACDataType for HouseBootSpecificHouseGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

