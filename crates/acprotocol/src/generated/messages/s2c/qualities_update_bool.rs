use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set or update an Object Boolean property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateBool")]
pub struct QualitiesUpdateBool {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyBool,
    #[serde(rename = "Value")]
    pub value: bool,
}

impl crate::readers::ACDataType for QualitiesUpdateBool {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyBool::try_from(read_u32(reader)?)?;
        let value = read_bool(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

