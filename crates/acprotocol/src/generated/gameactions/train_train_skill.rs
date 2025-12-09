use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Spend XP to raise a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkill")]
pub struct TrainTrainSkill {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

impl TrainTrainSkill {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let skill = SkillId::try_from(read_i32(reader)?)?;
        let experience = read_u32(reader)?;

        Ok(Self {
            skill,
            experience,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainSkill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainSkill::read(reader)
    }
}

