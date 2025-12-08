use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::common::*;
#[allow(unused_imports)]
use crate::enums::*;

// Sets an autonomy level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomyLevel")]
pub struct MovementAutonomyLevel {
    #[serde(rename = "AutonomyLevel")]
    pub autonomy_level: u32,
}

impl MovementAutonomyLevel {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let autonomy_level = read_u32(reader)?;
        align_dword(reader)?;

        Ok(Self {
            autonomy_level,
        })
    }
}

impl crate::readers::ACDataType for MovementAutonomyLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementAutonomyLevel::read(reader)
    }
}

