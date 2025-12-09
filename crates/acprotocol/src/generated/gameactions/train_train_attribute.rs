use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Spend XP to raise an attribute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainAttribute")]
pub struct TrainTrainAttribute {
    #[serde(rename = "Type")]
    pub type_: AttributeId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

impl TrainTrainAttribute {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = AttributeId::try_from(read_u32(reader)?)?;
        let experience = read_u32(reader)?;

        Ok(Self {
            type_,
            experience,
        })
    }
}

impl crate::readers::ACDataType for TrainTrainAttribute {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        TrainTrainAttribute::read(reader)
    }
}

