use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::writers::ACWriter;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::writers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Spend XP to raise a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkill")]
pub struct TrainTrainSkill {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

impl crate::readers::ACDataType for TrainTrainSkill {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TrainTrainSkill").entered();

        #[cfg(feature = "tracing")]
        let _field_span_skill = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Skill", position = pos).entered()
        };
        let skill = SkillId::try_from(read_i32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skill);
        #[cfg(feature = "tracing")]
        let _field_span_experience = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Experience", position = pos).entered()
        };
        let experience = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_experience);

        Ok(Self {
            skill,
            experience,
        })
    }
}

impl crate::writers::ACWritable for TrainTrainSkill {
    fn write(&self, writer: &mut dyn ACWriter) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "write", r#type = "TrainTrainSkill").entered();

        write_i32(writer, self.skill.clone() as i32)?;
        write_u32(writer, self.experience)?;
        Ok(())
    }
}

