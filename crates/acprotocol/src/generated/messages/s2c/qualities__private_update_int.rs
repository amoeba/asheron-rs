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

// Set or update a Character Int property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateInt")]
pub struct QualitiesPrivateUpdateInt {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyInt,
    #[serde(rename = "Value")]
    pub value: i32,
}

impl QualitiesPrivateUpdateInt {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyInt::try_from(read_u32(reader)?)?;
        let value = read_i32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateInt {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateInt::read(reader)
    }
}

