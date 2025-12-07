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

// Sends an autonomous position
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_AutonomousPosition")]
pub struct MovementAutonomousPosition {
    #[serde(rename = "Position")]
    pub position: AutonomousPositionPack,
}

impl MovementAutonomousPosition {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let position = AutonomousPositionPack::read(reader)?;

        Ok(Self {
            position,
        })
    }
}

impl crate::readers::ACDataType for MovementAutonomousPosition {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementAutonomousPosition::read(reader)
    }
}

