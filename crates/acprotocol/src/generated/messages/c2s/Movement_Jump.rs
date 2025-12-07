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

