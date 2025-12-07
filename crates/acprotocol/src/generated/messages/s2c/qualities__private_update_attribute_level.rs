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

// Set or update a Character Attribute Level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateAttributeLevel")]
pub struct QualitiesPrivateUpdateAttributeLevel {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: u32,
}

impl QualitiesPrivateUpdateAttributeLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = read_u32(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttributeLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateAttributeLevel::read(reader)
    }
}

