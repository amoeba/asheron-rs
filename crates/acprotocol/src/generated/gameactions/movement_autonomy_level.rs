use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Sets an autonomy level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomyLevel")]
pub struct MovementAutonomyLevel {
    #[serde(rename = "AutonomyLevel")]
    pub autonomy_level: u32,
}

impl crate::readers::ACDataType for MovementAutonomyLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let autonomy_level = read_u32(reader)?;
        align_dword(reader)?;

        Ok(Self {
            autonomy_level,
        })
    }
}

