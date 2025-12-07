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

// Set or update an Object String property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateString")]
pub struct QualitiesUpdateString {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyString,
    #[serde(rename = "Value")]
    pub value: String,
}

impl QualitiesUpdateString {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyString::try_from(read_u32(reader)?)?;
        let __alignment_marker_align_dword = align_dword(reader)?;
        let value = read_string(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateString {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateString::read(reader)
    }
}

