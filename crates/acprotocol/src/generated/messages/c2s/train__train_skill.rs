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

