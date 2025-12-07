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

// Set or update a Character Skill state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateUpdateSkillAC")]
pub struct QualitiesPrivateUpdateSkillAC {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: SkillAdvancementClass,
}

impl QualitiesPrivateUpdateSkillAC {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = SkillAdvancementClass::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            key,
            value,
        })
    }
}

impl crate::readers::ACDataType for QualitiesPrivateUpdateSkillAC {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        QualitiesPrivateUpdateSkillAC::read(reader)
    }
}

