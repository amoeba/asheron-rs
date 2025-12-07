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

// Set or update a Character Attribute value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute")]
pub struct QualitiesUpdateAttribute {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: AttributeId,
    #[serde(rename = "Value")]
    pub value: AttributeInfo,
}

impl QualitiesUpdateAttribute {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = AttributeId::try_from(read_u32(reader)?)?;
        let value = AttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateAttribute {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateAttribute::read(reader)
    }
}

