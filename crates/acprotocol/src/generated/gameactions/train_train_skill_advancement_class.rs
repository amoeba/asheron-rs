use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Spend skill credits to train a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkillAdvancementClass")]
pub struct TrainTrainSkillAdvancementClass {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Credits")]
    pub credits: u32,
}

impl TrainTrainSkillAdvancementClass {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let skill = SkillId::try_from(read_i32(reader)?)?;
        let credits = read_u32(reader)?;

        Ok(Self {
            skill,
            credits,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainSkillAdvancementClass {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainSkillAdvancementClass::read(reader)
    }
}

