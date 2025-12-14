use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl crate::readers::ACDataType for FragmentHeader {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

