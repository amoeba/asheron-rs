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

// A single fragment containing header and payload data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    #[serde(rename = "Header")]
    pub header: FragmentHeader,
    #[serde(rename = "Data")]
    pub data: Vec<byte>,
}

impl Fragment {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let header = FragmentHeader::read(reader)?;
        let data = read_vec::<u8>(reader, *)?;

        Ok(Self {
            header,
            data,
        })
    }
}

impl crate::readers::ACDataType for Fragment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        Fragment::read(reader)
    }
}

