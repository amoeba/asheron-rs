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

// Set or update a Character Position property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdatePosition")]
pub struct QualitiesUpdatePosition {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: PropertyPosition,
    #[serde(rename = "Value")]
    pub value: Position,
}

impl QualitiesUpdatePosition {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = PropertyPosition::try_from(read_u32(reader)?)?;
        let value = Position::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdatePosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdatePosition::read(reader)
    }
}

