use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
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
        let data = {
                let mut data = Vec::new();
                let _ = reader.read_to_end(&mut data);
                Ok::<_, Box<dyn std::error::Error>>(data)
            }?;

        Ok(Self {
            header,
            data,
        })
    }
}

