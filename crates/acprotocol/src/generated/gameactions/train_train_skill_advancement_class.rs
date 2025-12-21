use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;
#[cfg(feature = "tracing")]
#[allow(unused_imports)]
use tracing::{span, Level};

// Spend skill credits to train a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainSkillAdvancementClass")]
pub struct TrainTrainSkillAdvancementClass {
    #[serde(rename = "Skill")]
    pub skill: SkillId,
    #[serde(rename = "Credits")]
    pub credits: u32,
}

impl crate::readers::ACDataType for TrainTrainSkillAdvancementClass {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "tracing")]
        let _span = tracing::span!(tracing::Level::DEBUG, "read", r#type = "TrainTrainSkillAdvancementClass").entered();

        #[cfg(feature = "tracing")]
        let _field_span_skill = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Skill", position = pos).entered()
        };
        let skill = SkillId::try_from(read_i32(reader)?)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_skill);
        #[cfg(feature = "tracing")]
        let _field_span_credits = {
            let pos = reader.stream_position().unwrap_or(0);
            tracing::span!(tracing::Level::TRACE, "field", name = "Credits", position = pos).entered()
        };
        let credits = read_u32(reader)?;
        #[cfg(feature = "tracing")]
        drop(_field_span_credits);

        Ok(Self {
            skill,
            credits,
        })
    }
}

