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
        let _ = align_dword(reader)?;

        Ok(Self {
            autonomy_level,
        })
    }
}

