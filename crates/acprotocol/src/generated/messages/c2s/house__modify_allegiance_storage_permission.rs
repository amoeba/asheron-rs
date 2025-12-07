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

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ModifyAllegianceStoragePermission")]
pub struct HouseModifyAllegianceStoragePermission {
    #[serde(rename = "Add")]
    pub add: bool,
}

impl HouseModifyAllegianceStoragePermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;

        Ok(Self {
            add,
        })
    }
}

impl crate::readers::ACDataType for HouseModifyAllegianceStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseModifyAllegianceStoragePermission::read(reader)
    }
}

