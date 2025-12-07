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

// Set or update a Character Vital value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateAttribute2nd")]
pub struct QualitiesUpdateAttribute2nd {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: VitalId,
    #[serde(rename = "Value")]
    pub value: SecondaryAttributeInfo,
}

impl QualitiesUpdateAttribute2nd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = VitalId::try_from(read_u32(reader)?)?;
        let value = SecondaryAttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesUpdateAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesUpdateAttribute2nd::read(reader)
    }
}

