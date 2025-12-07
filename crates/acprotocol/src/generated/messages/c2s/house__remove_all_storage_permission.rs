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

// Removes all storage permissions, /house storage remove_all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RemoveAllStoragePermission")]
pub struct HouseRemoveAllStoragePermission {}

impl HouseRemoveAllStoragePermission {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseRemoveAllStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseRemoveAllStoragePermission::read(reader)
    }
}

