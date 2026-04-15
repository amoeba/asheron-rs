use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Adds all to your storage permissions, /house storage add -all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AddAllStoragePermission")]
pub struct HouseAddAllStoragePermission {}

impl HouseAddAllStoragePermission {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseAddAllStoragePermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAddAllStoragePermission::read(reader)
    }
}

impl HouseAddAllStoragePermission {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for HouseAddAllStoragePermission {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        HouseAddAllStoragePermission::write(self, writer)
    }
}

