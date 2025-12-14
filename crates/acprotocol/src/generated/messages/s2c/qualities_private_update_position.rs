use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set or update a Character Position property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdatePosition")]
pub struct QualitiesPrivateUpdatePosition {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: PropertyPosition,
    #[serde(rename = "Value")]
    pub value: Position,
}

impl crate::readers::ACDataType for QualitiesPrivateUpdatePosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = PropertyPosition::try_from(read_u32(reader)?)?;
        let value = Position::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

