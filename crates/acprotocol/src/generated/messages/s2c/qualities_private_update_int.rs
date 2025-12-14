use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

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

impl crate::readers::ACDataType for QualitiesPrivateUpdateInt {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
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

