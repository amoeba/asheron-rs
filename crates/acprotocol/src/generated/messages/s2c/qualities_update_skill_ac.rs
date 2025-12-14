use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Set or update a Character Skill state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_UpdateSkillAC")]
pub struct QualitiesUpdateSkillAC {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Key")]
    pub key: SkillId,
    #[serde(rename = "Value")]
    pub value: SkillAdvancementClass,
}

impl crate::readers::ACDataType for QualitiesUpdateSkillAC {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let sequence = read_u8(reader)?;
        let object_id = ObjectId::read(reader)?;
        let key = SkillId::try_from(read_i32(reader)?)?;
        let value = SkillAdvancementClass::try_from(read_u32(reader)?)?;

        Ok(Self {
            sequence,
            object_id,
            key,
            value,
        })
    }
}

