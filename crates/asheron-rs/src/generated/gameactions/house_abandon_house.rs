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

// Abandons your house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AbandonHouse")]
pub struct HouseAbandonHouse {}

impl HouseAbandonHouse {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for HouseAbandonHouse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        HouseAbandonHouse::read(reader)
    }
}

impl HouseAbandonHouse {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for HouseAbandonHouse {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        HouseAbandonHouse::write(self, writer)
    }
}

