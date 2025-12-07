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

