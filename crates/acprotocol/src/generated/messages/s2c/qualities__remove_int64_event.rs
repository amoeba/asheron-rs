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

// Remove an int64 property from an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_RemoveInt64Event")]
pub struct QualitiesRemoveInt64Event {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Type")]
    pub type_: PropertyInt64,
}

impl QualitiesRemoveInt64Event {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let type_ = PropertyInt64::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            type_,
        })
    }
}

impl crate::readers::ACDataType for QualitiesRemoveInt64Event {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesRemoveInt64Event::read(reader)
    }
}

