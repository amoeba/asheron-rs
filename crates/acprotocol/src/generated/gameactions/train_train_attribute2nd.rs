use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Spend XP to raise a vital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainAttribute2nd")]
pub struct TrainTrainAttribute2nd {
    #[serde(rename = "Type")]
    pub type_: VitalId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

impl TrainTrainAttribute2nd {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = VitalId::try_from(read_u32(reader)?)?;
        let experience = read_u32(reader)?;

        Ok(Self {
            type_,
            experience,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainAttribute2nd {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainAttribute2nd::read(reader)
    }
}

