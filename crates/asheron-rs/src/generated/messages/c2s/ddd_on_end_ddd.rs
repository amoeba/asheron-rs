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

// Ends DDD update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_OnEndDDD")]
pub struct DDDOnEndDDD {}

impl DDDOnEndDDD {
    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

impl crate::readers::ACDataType for DDDOnEndDDD {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDOnEndDDD::read(reader)
    }
}

impl DDDOnEndDDD {
    pub fn write(&self, _writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl crate::writers::ACWritable for DDDOnEndDDD {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        DDDOnEndDDD::write(self, writer)
    }
}

