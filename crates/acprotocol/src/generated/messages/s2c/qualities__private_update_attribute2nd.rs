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
#[serde(rename = "Qualities_PrivateUpdateAttribute2nd")]
pub struct QualitiesPrivateUpdateAttribute2nd {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: VitalId,
    #[serde(rename = "Value")]
    pub value: SecondaryAttributeInfo,
}

impl QualitiesPrivateUpdateAttribute2nd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = VitalId::try_from(read_u32(reader)?)?;
        let value = SecondaryAttributeInfo::read(reader)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateAttribute2nd::read(reader)
    }
}

