use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// A single fragment containing header and payload data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fragment {
    #[serde(rename = "Header")]
    pub header: FragmentHeader,
    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

impl crate::readers::ACDataType for Fragment {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let header = FragmentHeader::read(reader)?;
        let data = (|| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                let mut data = Vec::new();
                let _ = reader.read_to_end(&mut data);
                Ok(data)
            })()?;

        Ok(Self {
            header,
            data,
        })
    }
}

