use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ModifyAllegianceStoragePermission")]
pub struct HouseModifyAllegianceStoragePermission {
    #[serde(rename = "Add")]
    pub add: bool,
}

impl crate::readers::ACDataType for HouseModifyAllegianceStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let add = read_bool(reader)?;

        Ok(Self {
            add,
        })
    }
}

