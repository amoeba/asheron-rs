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

// TODO
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationResponseMessage")]
pub struct DDDInterrogationResponseMessage {
    #[serde(rename = "Language")]
    pub language: u32,
    #[serde(rename = "Files")]
    pub files: PackableList<long>,
}

impl DDDInterrogationResponseMessage {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let language = read_u32(reader)?;
        let files = read_packable_list::<i64>(reader)?;

        Ok(Self {
            language,
            files,
        })
    }
}

impl crate::readers::ACDataType for DDDInterrogationResponseMessage {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        DDDInterrogationResponseMessage::read(reader)
    }
}

