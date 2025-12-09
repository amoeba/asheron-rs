use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Modify whether allegiance members are guests, /house guest add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ModifyAllegianceGuestPermission")]
pub struct HouseModifyAllegianceGuestPermission {
    #[serde(rename = "Add")]
    pub add: bool,
}

impl HouseModifyAllegianceGuestPermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;

        Ok(Self {
            add,
        })
    }
}

impl crate::readers::ACDataType for HouseModifyAllegianceGuestPermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseModifyAllegianceGuestPermission::read(reader)
    }
}

