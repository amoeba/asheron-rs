use serde::{Serialize, Deserialize};
use std::io::Read;
use crate::readers::ACReader;
use crate::readers::*;
use crate::types::*;
use crate::enums::*;
use super::*;

// Performs a jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_Jump")]
pub struct MovementJump {
    #[serde(rename = "Jump")]
    pub jump: JumpPack,
}

impl MovementJump {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let jump = JumpPack::read(reader)?;

        Ok(Self {
            jump,
        })
    }
}

impl crate::readers::ACDataType for MovementJump {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        MovementJump::read(reader)
    }
}

