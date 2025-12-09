use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ChangeStoragePermission")]
pub struct HouseChangeStoragePermission {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
    #[serde(rename = "HasPermission")]
    pub has_permission: bool,
}

impl HouseChangeStoragePermission {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;
        let has_permission = read_bool(reader)?;

        Ok(Self {
            guest_name,
            has_permission,
        })
    }
}

impl crate::readers::ACDataType for HouseChangeStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseChangeStoragePermission::read(reader)
    }
}

