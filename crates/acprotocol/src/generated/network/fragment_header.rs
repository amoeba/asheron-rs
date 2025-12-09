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

// Header for packet fragments used in network communication
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FragmentHeader {
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Count")]
    pub count: u16,
    #[serde(rename = "Index")]
    pub index: u16,
}

impl FragmentHeader {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u32(reader)?;
        let id = read_u32(reader)?;
        let count = read_u16(reader)?;
        let index = read_u16(reader)?;

        Ok(Self {
            sequence,
            id,
            count,
            index,
        })
    }
}

impl crate::readers::ACDataType for FragmentHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        FragmentHeader::read(reader)
    }
}

